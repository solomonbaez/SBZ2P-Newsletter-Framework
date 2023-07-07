use actix_web_flash_messages::FlashMessage;

#[derive(Debug)]
pub struct IdempotencyKey(String);

impl TryFrom<String> for IdempotencyKey {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.is_empty() {
            // hard reset option:
            // anyhow::bail!("The idempotency key cannot be empty");
            FlashMessage::error("The idempotency key cannot be empty!").send();
            return Err(anyhow::anyhow!("The idempotency key cannot be empty!"));
        }
        Ok(Self(s))
    }
}

impl From<IdempotencyKey> for String {
    fn from(k: IdempotencyKey) -> Self {
        k.0
    }
}

impl AsRef<str> for IdempotencyKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
