use actix_web::HttpResponse;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use log::{error, info};

#[derive(Debug, Deserialize)]
struct GithubRepo {
    name: String,
    description: Option<String>,
    html_url: String,
    language: Option<String>,
    stargazers_count: u32,
    forks_count: u32,
    updated_at: String,
    topics: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct RepoResponse {
    name: String,
    description: String,
    url: String,
    language: String,
    stars: u32,
    forks: u32,
    updated_at: String,
    topics: Vec<String>,
}

impl From<GithubRepo> for RepoResponse {
    fn from(repo: GithubRepo) -> Self {
        RepoResponse {
            name: repo.name,
            description: repo.description.unwrap_or_default(),
            url: repo.html_url,
            language: repo.language.unwrap_or_else(|| "Unknown".to_string()),
            stars: repo.stargazers_count,
            forks: repo.forks_count,
            updated_at: repo.updated_at,
            topics: repo.topics.unwrap_or_default(),
        }
    }
}

pub async fn get_repos() -> HttpResponse {
    let username = env::var("GITHUB_USERNAME").unwrap_or_else(|_| "System-End".to_string());
    let url = format!("https://api.github.com/users/{}/repos?sort=updated&per_page=10", username);
    
    info!("Fetching repos from GitHub for user: {}", username);
    
    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "personal-site-rust")
        .send()
        .await;
    
    match response {
        Ok(res) => {
            if res.status().is_success() {
                match res.json::<Vec<GithubRepo>>().await {
                    Ok(repos) => {
                        let repos: Vec<RepoResponse> = repos.into_iter().map(Into::into).collect();
                        HttpResponse::Ok().json(repos)
                    }
                    Err(e) => {
                        error!("Failed to parse GitHub response: {}", e);
                        HttpResponse::InternalServerError().json(serde_json::json!({
                            "error": "Failed to parse GitHub response"
                        }))
                    }
                }
            } else {
                error!("GitHub API returned error status: {}", res.status());
                HttpResponse::BadGateway().json(serde_json::json!({
                    "error": "GitHub API error"
                }))
            }
        }
        Err(e) => {
            error!("Failed to fetch from GitHub: {}", e);
            HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Unable to reach GitHub"
            }))
        }
    }
}
