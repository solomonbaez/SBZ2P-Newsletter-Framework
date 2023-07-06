use crate::authentication::UserId;
use crate::idempotency::IdempotencyKey;
use crate::utils::{e400, see_other};
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use sqlx::PgPool;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct FormData {
    idempotency_key: String,
    validity: String,
}

#[tracing::instrument(
    name="Change a key state"
    skip_all,
    fields(user_id=%*user_id)
)]
pub async fn change_key_state(
    form: web::Form<FormData>,
    user_id: web::ReqData<UserId>,
    connection_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();

    let FormData {
        idempotency_key,
        validity,
    } = form.0;

    let idempotency_key: IdempotencyKey = idempotency_key.try_into().map_err(e400)?;

    match key_state(*user_id, idempotency_key, &connection_pool, &validity).await {
        Ok(_) => {
            FlashMessage::info("The key state has been changed.").send();
        }
        Err(_) => {
            FlashMessage::error("Failed to change the key state.").send();
        }
    }

    let response = see_other("/admin/settings");
    Ok(response)
}

async fn key_state(
    user_id: Uuid,
    idempotency_key: IdempotencyKey,
    connection_pool: &PgPool,
    validity: &str,
) -> Result<(), anyhow::Error> {
    let key_validity = match validity {
        "1" => true,
        "0" => false,
        &_ => false,
    };

    let changed_key = sqlx::query!(
        r#"
        UPDATE idempotency
        SET 
            validity = $3
        WHERE
            user_id = $1 
            AND idempotency_key = $2
        "#,
        user_id,
        idempotency_key.as_ref(),
        key_validity,
    )
    .execute(connection_pool)
    .await?
    .rows_affected();

    if changed_key > 0 {
        Ok(())
    } else {
        Err(anyhow::anyhow!("No matching key found."))
    }
}
