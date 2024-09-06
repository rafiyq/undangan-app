use askama_axum::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub title: String,
    pub guestname: String,
    pub datetime_remaining: u8,
}