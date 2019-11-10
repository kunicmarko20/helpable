use serde::export::fmt::Error;
use serde::export::Formatter;
use std::fmt::Display;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BuildStatus {
    /// Successful build
    Success,
    /// Unstable build
    Unstable,
    /// Failed build
    Failure,
    /// Not yet built
    NotBuilt,
    /// Aborted build
    Aborted,
}

impl Display for BuildStatus {
    fn fmt(&self, formater: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            formater,
            "{}",
            match self {
                BuildStatus::Aborted => "Build was aborted.",
                BuildStatus::Failure => "Build failed.",
                BuildStatus::Unstable => "Build was unstable.",
                BuildStatus::NotBuilt => "Build is still building.",
                BuildStatus::Success => "Build was successful.",
            }
        )
    }
}
