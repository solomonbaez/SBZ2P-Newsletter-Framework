use crate::authentication::UserId;
use crate::idempotency::IdempotencyKey;
use crate::utils::e400;
use actix_web::{web, HttpResponse};
// use actix_web_flash_messages::FlashMessage;
// use anyhow::Context;
// use uuid::Uuid;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct UserKey {
    idempotency_key: String,
}

#[tracing::instrument(
    name="Revoke an idempotency key"
    skip_all,
    fields(user_id=%*user_id)
)]
pub async fn revoke_key(
    user_id: web::ReqData<UserId>,
    key: web::Form<UserKey>,
) -> Result<HttpResponse, actix_web::Error> {
    let _user_id = user_id.into_inner();
    let idempotency_key = key.0;

    let _idempotency_key: IdempotencyKey = idempotency_key.try_into().map_err(e400)?;

    // TODO: Need to insert an idempotency validity column
    //       -> bool for deactivation/reactivation

    // TODO: Need to create a idempotency/persistence fn to 
    //       change the status of keys per user-input

}