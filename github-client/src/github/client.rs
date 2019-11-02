use super::payload::PullRequestCommitPayload;
use super::payload::PullRequestPayload;
use crate::github::payload::{BranchPayload, ErrorPayload, SearchPayload};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, Response,
};

pub type Result<T> = ::std::result::Result<T, ErrorPayload>;
pub type GenericResponse = Result<Response>;

pub struct GithubClient {
    client: Client,
}

impl GithubClient {
    const BASE_URL: &'static str = "https://api.github.com/";

    pub fn new(token: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_bytes(("token ".to_string() + &token).as_bytes()).unwrap(),
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        GithubClient { client }
    }

    pub fn create_pull_request(
        &self,
        repository_name: &str,
        body: String,
    ) -> Result<PullRequestPayload> {
        let mut response = self
            .client
            .post(&format!(
                "{}repos/{}/pulls",
                Self::BASE_URL,
                repository_name,
            ))
            .body(body)
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(response.json().unwrap())
    }

    pub fn update_pull_request(
        &self,
        repository_name: &str,
        pull_request_number: u64,
        body: String,
    ) -> GenericResponse {
        let mut response = self
            .client
            .patch(&format!(
                "{}repos/{}/pulls/{}",
                Self::BASE_URL,
                repository_name,
                pull_request_number,
            ))
            .body(body)
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(response)
    }

    pub fn create_comment(
        &self,
        repository_name: &str,
        issue_number: u64,
        body: String,
    ) -> GenericResponse {
        let mut response = self
            .client
            .post(&format!(
                "{}repos/{}/issues/{}/comments",
                Self::BASE_URL,
                repository_name,
                issue_number,
            ))
            .body(body)
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(response)
    }

    pub fn merge_pull_request(
        &self,
        repository_name: &str,
        pull_request_number: u64,
        body: String,
    ) -> GenericResponse {
        let mut response = self
            .client
            .put(&format!(
                "{}repos/{}/pulls/{}/merge",
                Self::BASE_URL,
                repository_name,
                pull_request_number,
            ))
            .body(body)
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(response)
    }

    pub fn pull_request_info(
        &self,
        repository_name: &str,
        pull_request_number: u64,
    ) -> Result<PullRequestPayload> {
        let mut response = self
            .client
            .get(&format!(
                "{}repos/{}/pulls/{}",
                Self::BASE_URL,
                repository_name,
                pull_request_number,
            ))
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(response.json().unwrap())
    }

    pub fn pull_request_commits(
        &self,
        repository_name: &str,
        pull_request_number: u64,
    ) -> Result<Vec<PullRequestCommitPayload>> {
        let mut response = self
            .client
            .get(&format!(
                "{}repos/{}/pulls/{}/commits",
                Self::BASE_URL,
                repository_name,
                pull_request_number,
            ))
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(response.json().unwrap())
    }

    pub fn list_pull_requests(&self, repository_name: &str) -> Result<Vec<PullRequestPayload>> {
        let mut response = self
            .client
            .get(&format!(
                "{}repos/{}/pulls",
                Self::BASE_URL,
                repository_name,
            ))
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(response.json().unwrap())
    }

    pub fn approve_pull_requests(
        &self,
        repository_name: &str,
        pull_request_number: u64,
    ) -> GenericResponse {
        let mut response = self
            .client
            .post(&format!(
                "{}repos/{}/pulls/{}/reviews",
                Self::BASE_URL,
                repository_name,
                pull_request_number,
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

    pub fn branch_info(&self, repository_name: &str, branch: &str) -> Result<BranchPayload> {
        let mut response = self
            .client
            .get(&format!(
                "{}repos/{}/branches/{}",
                Self::BASE_URL,
                repository_name,
                branch,
            ))
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(response.json().unwrap())
    }

    pub fn search_pull_requests(&self, repository_name: &str, term: &str) -> Result<SearchPayload> {
        let mut response = self
            .client
            .get(&format!(
                "{}search/issues?q={}+is:pr+repo:{}",
                Self::BASE_URL,
                term,
                repository_name,
            ))
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(response.json().unwrap())
    }
}
