use actix_web::{web, HttpResponse};
use secrecy::Secret;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

pub async fn change_password(_form: web::Form<FormData>) -> Result<HttpResponse, actix_web::Error> {
    todo!()
}
