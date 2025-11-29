use askama::Template;

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {
    pub title: String,
    pub name: String,
    pub pronouns: String,
    pub role: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "projects.html")]
pub struct ProjectsTemplate {
    pub title: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFoundTemplate {
    pub title: String,
    pub active_page: String,
}
