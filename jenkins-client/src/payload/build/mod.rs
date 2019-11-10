use self::build_data::{deserialize_build_data, BuildData};
use self::build_status::BuildStatus;
use serde::Deserialize;
use std::fmt::{Display, Error, Formatter};

pub mod build_data;
pub mod build_status;

#[derive(Debug, Deserialize)]
pub struct Build {
    #[serde(deserialize_with = "deserialize_build_data")]
    #[serde(rename = "actions")]
    build_data: Option<BuildData>,
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

    pub fn build_data(&self) -> &Option<BuildData> {
        &self.build_data
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
