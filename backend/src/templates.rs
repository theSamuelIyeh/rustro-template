use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HelloTemplate {
    pub name: String,
}
