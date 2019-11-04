use serde::export::fmt::Error;
use serde::export::Formatter;
use std::fmt::Display;

#[derive(Debug, Deserialize)]
pub struct Build {
    building: bool,
    result: Option<BuildStatus>,
}

impl Build {
    pub fn is_success(&self) -> bool {
        if self.building || self.result.is_none() {
            return false;
        }

        let result = self.result.as_ref().unwrap();

        result.eq(&BuildStatus::Success)
    }

    fn message(&self) -> String {
        if self.building {
            return "Build didn't finish yet.".to_string();
        }

        if self.result.is_none() {
            panic!("Something is wrong as this should never happen.");
        }

        self.result.as_ref().unwrap().to_string()
    }
}

impl Display for Build {
    fn fmt(&self, formater: &mut Formatter<'_>) -> Result<(), Error> {
        write!(formater, "{}", self.message())
    }
}

/// Status of a build
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
