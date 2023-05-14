use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(parameters, pool))]
pub async fn confirm(parameters: web::Query<Parameters>, pool: web::Data<PgPool>) -> HttpResponse {
    let id = match get_subscriber_id_from_token(&parameters.subscription_token, &pool).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match id {
        None => HttpResponse::InternalServerError().finish(),
        Some(subscriber_id) => {
            if confirm_subscriber(subscriber_id, &pool).await.is_err() {
                return HttpResponse::InternalServerError().finish();
            }
            HttpResponse::Ok().finish()
        }
    }
}

#[tracing::instrument(name = "Get subscriber id from token", skip(subscription_token, pool))]
pub async fn get_subscriber_id_from_token(
    subscription_token: &str,
    pool: &PgPool,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT subscriber_id FROM subscription_tokens \
        WHERE subscription_token = $1",
        subscription_token,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query {:?}", e);
        e
    })?;
    Ok(result.map(|r| r.subscriber_id))
}

#[tracing::instrument(name = "Confirm subscriber", skip(subscriber_id, pool))]
pub async fn confirm_subscriber(subscriber_id: Uuid, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE subscriptions SET status = 'confirmed' WHERE id = $1",
        subscriber_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query {:?}", e);
        e
    })?;
    Ok(())
}
