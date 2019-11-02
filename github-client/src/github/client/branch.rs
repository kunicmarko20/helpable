use super::{Result, BASE_URL};
use crate::github::payload::BranchPayload;
use reqwest::Client;

pub struct BranchClient<'a> {
    pub(super) client: &'a Client,
}

impl<'a> BranchClient<'a> {
    pub fn get(&self, repository_name: &str, branch: &str) -> Result<BranchPayload> {
        let mut response = self
            .client
            .get(&format!(
                "{}repos/{}/branches/{}",
                BASE_URL, repository_name, branch,
            ))
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(response.json().unwrap())
    }
}
