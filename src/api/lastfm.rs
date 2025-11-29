use actix_web::HttpResponse;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use log::{error, info};

#[derive(Debug, Deserialize)]
struct LastFmImage {
    #[serde(rename = "#text")]
    url: String,
    size: String,
}

#[derive(Debug, Deserialize)]
struct LastFmArtist {
    #[serde(rename = "#text")]
    name: String,
}

#[derive(Debug, Deserialize)]
struct LastFmNowPlaying {
    nowplaying: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LastFmTrack {
    name: String,
    artist: LastFmArtist,
    image: Vec<LastFmImage>,
    #[serde(rename = "@attr")]
    attr: Option<LastFmNowPlaying>,
}

#[derive(Debug, Deserialize)]
struct LastFmRecentTracks {
    track: Vec<LastFmTrack>,
}

#[derive(Debug, Deserialize)]
struct LastFmResponse {
    recenttracks: LastFmRecentTracks,
}

#[derive(Debug, Serialize)]
pub struct CurrentTrack {
    name: String,
    artist: String,
    image: String,
    is_playing: bool,
}

#[derive(Debug, Serialize)]
pub struct LastFmApiResponse {
    current_track: Option<CurrentTrack>,
    error: Option<String>,
}

pub async fn get_current_track() -> HttpResponse {
    let api_key = match env::var("LASTFM_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return HttpResponse::Ok().json(LastFmApiResponse {
                current_track: None,
                error: Some("Last.fm API key not configured".to_string()),
            });
        }
    };

    let username = env::var("LASTFM_USERNAME").unwrap_or_else(|_| "EndofTimee".to_string());
    
    let url = format!(
        "https://ws.audioscrobbler.com/2.0/?method=user.getrecenttracks&user={}&api_key={}&format=json&limit=1",
        username, api_key
    );
    
    info!("Fetching Last.fm data for user: {}", username);
    
    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "personal-site-rust")
        .send()
        .await;
    
    match response {
        Ok(res) => {
            if res.status().is_success() {
                match res.json::<LastFmResponse>().await {
                    Ok(data) => {
                        if data.recenttracks.track.is_empty() {
                            return HttpResponse::Ok().json(LastFmApiResponse {
                                current_track: None,
                                error: None,
                            });
                        }

                        let track = &data.recenttracks.track[0];
                        
                        // Check if currently playing
                        let is_playing = track.attr
                            .as_ref()
                            .and_then(|a| a.nowplaying.as_ref())
                            .map(|np| np == "true")
                            .unwrap_or(false);

                        // Get the large image or fallback to the last one
                        let image = track.image
                            .iter()
                            .find(|img| img.size == "large")
                            .or_else(|| track.image.last())
                            .map(|img| img.url.clone())
                            .unwrap_or_default();

                        let current_track = CurrentTrack {
                            name: track.name.clone(),
                            artist: track.artist.name.clone(),
                            image,
                            is_playing,
                        };

                        HttpResponse::Ok().json(LastFmApiResponse {
                            current_track: Some(current_track),
                            error: None,
                        })
                    }
                    Err(e) => {
                        error!("Failed to parse Last.fm response: {}", e);
                        HttpResponse::Ok().json(LastFmApiResponse {
                            current_track: None,
                            error: Some("Failed to parse Last.fm response".to_string()),
                        })
                    }
                }
            } else {
                error!("Last.fm API returned error status: {}", res.status());
                HttpResponse::Ok().json(LastFmApiResponse {
                    current_track: None,
                    error: Some("Last.fm API error".to_string()),
                })
            }
        }
        Err(e) => {
            error!("Failed to fetch from Last.fm: {}", e);
            HttpResponse::Ok().json(LastFmApiResponse {
                current_track: None,
                error: Some("Unable to reach Last.fm".to_string()),
            })
        }
    }
}
