use crate::{templates, utils};
use actix_web::{get, Responder};

#[get("/")]
pub async fn hello_world() -> impl Responder {
    utils::render(
        templates::HelloTemplate {
            name: String::from("samuel"),
        },
        String::from("index"),
    )
    .await
    // HelloTemplate{name: String::from("samuel")}
}
