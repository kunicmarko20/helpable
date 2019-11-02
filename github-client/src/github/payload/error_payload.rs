use crate::github::payload::parts::Error;
use serde::export::Formatter;
use std::fmt::Display;
use std::fmt::Result;

#[derive(Deserialize, Debug)]
pub struct ErrorPayload {
    message: String,
    errors: Option<Vec<Error>>,
}

impl ErrorPayload {
    fn message(&self) -> String {
        let mut message = "Request to Github failed with error(s): ".to_string() + &self.message;

        if self.errors.is_none() {
            return message;
        }

        let errors = self.errors.as_ref().unwrap();

        for error in errors {
            message = message + &error.message + "; ";
        }

        message
    }
}

impl Display for ErrorPayload {
    fn fmt(&self, formater: &mut Formatter<'_>) -> Result {
        write!(formater, "{}", self.message())
    }
}
