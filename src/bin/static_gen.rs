use askama::Template;
use std::fs;
use std::path::Path;

// Import templates from the lib
#[path = "../templates.rs"]
mod templates;

use templates::{AboutTemplate, ProjectsTemplate, NotFoundTemplate};

fn main() -> std::io::Result<()> {
    // Create dist directory
    let dist_dir = Path::new("dist");
    if dist_dir.exists() {
        fs::remove_dir_all(dist_dir)?;
    }
    fs::create_dir_all(dist_dir)?;
    
    // Generate About page (index.html)
    let about = AboutTemplate {
        title: "About Me - EndofTimee".to_string(),
        name: "End".to_string(),
        pronouns: "They/It/She".to_string(),
        role: "Programmer & Streamer".to_string(),
        active_page: "about".to_string(),
    };
    fs::write(dist_dir.join("index.html"), about.render().unwrap())?;
    println!("Generated: dist/index.html");
    
    // Generate Projects page
    let projects = ProjectsTemplate {
        title: "My Projects - EndofTimee".to_string(),
        active_page: "projects".to_string(),
    };
    fs::create_dir_all(dist_dir.join("projects"))?;
    fs::write(dist_dir.join("projects/index.html"), projects.render().unwrap())?;
    println!("Generated: dist/projects/index.html");
    
    // Generate 404 page
    let not_found = NotFoundTemplate {
        title: "404 - Page Not Found".to_string(),
        active_page: "".to_string(),
    };
    fs::write(dist_dir.join("404.html"), not_found.render().unwrap())?;
    println!("Generated: dist/404.html");
    
    // Copy static files
    copy_dir_all("static", dist_dir.join("static"))?;
    println!("Copied: static/ -> dist/static/");
    
    // Copy public files
    copy_dir_all("public", dist_dir.join("public"))?;
    println!("Copied: public/ -> dist/public/");
    
    println!("\nBuild complete! Deploy dist/ to Cloudflare Pages");
    
    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
