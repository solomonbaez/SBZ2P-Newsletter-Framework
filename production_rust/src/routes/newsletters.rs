use actix_web::{web, HttpResponse};

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct BodyData {
    title: String,
    content: Content,
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct Content {
    text: String,
    html: String,
}

pub async fn publish_newsletter(_body: web::Json<BodyData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
