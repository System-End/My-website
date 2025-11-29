use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, middleware};
use askama::Template;
use log::info;
use std::env;

mod api;
mod templates;

use templates::{AboutTemplate, ProjectsTemplate, NotFoundTemplate};

// Route handlers
async fn about_page() -> HttpResponse {
    let template = AboutTemplate {
        title: "About Me - EndofTimee".to_string(),
        name: "End".to_string(),
        pronouns: "They/It/She".to_string(),
        role: "Programmer & Streamer".to_string(),
        active_page: "about".to_string(),
    };
    
    match template.render() {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(_) => HttpResponse::InternalServerError().body("Template rendering error"),
    }
}

async fn projects_page() -> HttpResponse {
    let template = ProjectsTemplate {
        title: "My Projects - EndofTimee".to_string(),
        active_page: "projects".to_string(),
    };
    
    match template.render() {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(_) => HttpResponse::InternalServerError().body("Template rendering error"),
    }
}

async fn not_found() -> HttpResponse {
    let template = NotFoundTemplate {
        title: "404 - Page Not Found".to_string(),
        active_page: "".to_string(),
    };
    
    match template.render() {
        Ok(body) => HttpResponse::NotFound().content_type("text/html").body(body),
        Err(_) => HttpResponse::InternalServerError().body("Template rendering error"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file if present
    dotenv::dotenv().ok();
    
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a valid number");
    
    info!("Starting server at http://{}:{}", host, port);
    
    HttpServer::new(|| {
        App::new()
            // Enable logger middleware
            .wrap(middleware::Logger::default())
            // API routes
            .service(
                web::scope("/api")
                    .route("/repos", web::get().to(api::github::get_repos))
                    .route("/health", web::get().to(api::health::health_check))
            )
            // Static files (CSS, JS, images)
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(fs::Files::new("/public", "./public").show_files_listing())
            // Page routes
            .route("/", web::get().to(about_page))
            .route("/projects", web::get().to(projects_page))
            // 404 fallback
            .default_service(web::route().to(not_found))
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
