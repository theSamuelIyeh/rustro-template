use actix_web::{http::header::ContentType, Either, Error, HttpResponse, Responder};
use reqwest::Client;
use std::env;

pub async fn render<T: Responder>(
    template: T,
    name: String,
) -> Result<Either<HttpResponse, T>, Error> {
    let dev_mode = env::var("RUSTRO_DEV").unwrap_or_default() == "true";
    let dev_port = "3000".to_string();

    if dev_mode {
        let client = Client::new();
        let url: String;
        if name == "index" {
            url = format!("http://localhost:{}/", dev_port);
        } else {
            url = format!("http://localhost:{}/{}.html", dev_port, name);
        }
        let resp = client.get(&url).send().await.expect("error getting page");
        let body = resp
            .text()
            .await
            .expect("conversion error from res to text");
        let template_text = body.replace(
            r#"src="/"#,
            &format!(r#"src="http://localhost:{}/"#, dev_port),
        );

        Ok(Either::Left(
            HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(template_text),
        ))
    } else {
        Ok(Either::Right(template))
    }
}
