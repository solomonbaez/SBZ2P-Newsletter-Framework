use validator::validate_email;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid subscriber email.", s))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claims::assert_err;

    #[test]
    fn reject_empty_email() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn reject_missing_at_email() {
        let email = "carthage.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn reject_missing_subject_email() {
        let email = "@phoenecian.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
}