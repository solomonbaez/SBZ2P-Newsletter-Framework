use crate::authentication::UserId;
use crate::idempotency::IdempotencyKey;
use crate::utils::{e400, e500};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use actix_web_flash_messages::FlashMessage;
use anyhow::Context;
// use uuid::Uuid;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct UserKey {
    idempotency_key: String,
}

#[tracing::instrument(
    name="Change a key state"
    skip_all,
    fields(user_id=%*user_id)
)]
pub async fn change_key_state(
    user_id: web::ReqData<UserId>,
    key: web::Form<UserKey>,
    connection_pool: web::Data<PgPool>,
    validity: Bool,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();

    let idempotency_key = key.0;
    let idempotency_key: IdempotencyKey = idempotency_key.try_into().map_err(e400)?;
    
    let validity = key_state(
        user_id, idempotency_key, connection_pool, validity
    )
    .await
    .context("Failed to change key state")
    .map_err(e500)?;

    FlashMessage::info(
        "The key state has been changed to {}", 
        validity
    )
    .send();

    // TODO: Need to insert an idempotency validity column
    //       -> bool for deactivation/reactivation
}

#[allow(dead_code)]
async fn key_state(
    user_id: Uuid,
    idempotency_key: Uuid,
    connection_pool: PgPool,
    validity: Bool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE idempotency
        SET 
            validity = $3,
        WHERE
            user_id = $1 
            AND idempotency_key = $2
        "#,
        user_id,
        idempotency_key.as_ref(),
        validity,
    )
    .execute(connection_pool)
    .await?;

    Ok(())
}