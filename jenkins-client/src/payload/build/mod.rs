use serde::export::fmt::Error;
use serde::export::Formatter;
use std::fmt::Display;
use self::build_status::BuildStatus;
use self::action::Action;

pub mod action;
pub mod build_status;

#[derive(Debug, Deserialize)]
pub struct Build {
    actions: Vec<Action>,
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

    pub fn build_data(&self) -> Option<Action> {
        for action in &self.actions {
            if let Action::BuildData { .. } = action {
                return Some(action.clone());
            }
        }

        None
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
