use actix_web::{web, HttpResponse};
use chrono::Utc;
use log;
use sqlx::PgPool;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pg_pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();

    log::info!(
        "{} - ADDING: email - '{}', name - '{}'",
        request_id,
        form.email,
        form.name
    );
    log::info!("{} - SAVING to database...", request_id);
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pg_pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!(
                "{} - SAVED: email - '{}', name - '{}', at - {}",
                request_id,
                form.email,
                form.name,
                Utc::now()
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("{} - FAILED to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
