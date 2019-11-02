use super::{GenericResponse, Result, BASE_URL};
use crate::github::payload::{PullRequestCommitPayload, PullRequestPayload, SearchPayload};
use reqwest::Client;

pub struct PullRequestClient<'a> {
    pub(super) client: &'a Client,
}

impl<'a> PullRequestClient<'a> {
    pub fn create(&self, repository_name: &str, body: String) -> Result<PullRequestPayload> {
        let mut response = self
            .client
            .post(&format!("{}repos/{}/pulls", BASE_URL, repository_name,))
            .body(body)
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(response.json().unwrap())
    }

    pub fn update(
        &self,
        repository_name: &str,
        pull_request_number: u64,
        body: String,
    ) -> GenericResponse {
        let mut response = self
            .client
            .patch(&format!(
                "{}repos/{}/pulls/{}",
                BASE_URL, repository_name, pull_request_number,
            ))
            .body(body)
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(response)
    }

    pub fn merge(
        &self,
        repository_name: &str,
        pull_request_number: u64,
        body: String,
    ) -> GenericResponse {
        let mut response = self
            .client
            .put(&format!(
                "{}repos/{}/pulls/{}/merge",
                BASE_URL, repository_name, pull_request_number,
            ))
            .body(body)
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(response)
    }

    pub fn get(
        &self,
        repository_name: &str,
        pull_request_number: u64,
    ) -> Result<PullRequestPayload> {
        let mut response = self
            .client
            .get(&format!(
                "{}repos/{}/pulls/{}",
                BASE_URL, repository_name, pull_request_number,
            ))
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(response.json().unwrap())
    }

    pub fn commits(
        &self,
        repository_name: &str,
        pull_request_number: u64,
    ) -> Result<Vec<PullRequestCommitPayload>> {
        let mut response = self
            .client
            .get(&format!(
                "{}repos/{}/pulls/{}/commits",
                BASE_URL, repository_name, pull_request_number,
            ))
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(response.json().unwrap())
    }

    pub fn approve(&self, repository_name: &str, pull_request_number: u64) -> GenericResponse {
        let mut response = self
            .client
            .post(&format!(
                "{}repos/{}/pulls/{}/reviews",
                BASE_URL, repository_name, pull_request_number,
            ))
            .body(
                json!({
                    "event": "APPROVE",
                })
                .to_string(),
            )
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(response)
    }

    pub fn all(&self, repository_name: &str) -> Result<Vec<PullRequestPayload>> {
        let mut response = self
            .client
            .get(&format!("{}repos/{}/pulls", BASE_URL, repository_name,))
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(response.json().unwrap())
    }

    pub fn search(&self, repository_name: &str, term: &str) -> Result<SearchPayload> {
        let mut response = self
            .client
            .get(&format!(
                "{}search/issues?q={}+is:pr+repo:{}",
                BASE_URL, term, repository_name,
            ))
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(response.json().unwrap())
    }
}
