use super::payload::PullRequestCommitPayload;
use super::payload::PullRequestPayload;
use crate::github::payload::{BranchPayload, ErrorPayload, SearchPayload};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, Response,
};

type ResultReqwest = reqwest::Result<Response>;

pub struct GithubClient {
    client: Client,
}

impl GithubClient {
    pub fn new(token: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_bytes(("token ".to_string() + &token).as_bytes()).unwrap(),
        );

        let client = Client::builder().default_headers(headers).build().unwrap();

        GithubClient { client }
    }

    pub fn create_pull_request(
        &self,
        repository_name: &str,
        body: String,
    ) -> Result<PullRequestPayload, ErrorPayload> {
        let mut response = self
            .client
            .post(&format!(
                "https://api.github.com/repos/{}/pulls",
                repository_name
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
    ) -> ResultReqwest {
        self.client
            .patch(&format!(
                "https://api.github.com/repos/{}/pulls/{}",
                repository_name, pull_request_number
            ))
            .body(body)
            .send()
    }

    pub fn create_comment(
        &self,
        repository_name: &str,
        issue_number: u64,
        body: String,
    ) -> ResultReqwest {
        self.client
            .post(&format!(
                "https://api.github.com/repos/{}/issues/{}/comments",
                repository_name, issue_number
            ))
            .body(body)
            .send()
    }

    pub fn merge_pull_request(
        &self,
        repository_name: &str,
        pull_request_number: u64,
        body: String,
    ) -> ResultReqwest {
        self.client
            .put(&format!(
                "https://api.github.com/repos/{}/pulls/{}/merge",
                repository_name, pull_request_number
            ))
            .body(body)
            .send()
    }

    pub fn pull_request_info(
        &self,
        repository_name: &str,
        pull_request_number: u64,
    ) -> PullRequestPayload {
        self.client
            .get(&format!(
                "https://api.github.com/repos/{}/pulls/{}",
                repository_name, pull_request_number
            ))
            .send()
            .unwrap()
            .json()
            .unwrap()
    }

    pub fn pull_request_commits(
        &self,
        repository_name: &str,
        pull_request_number: u64,
    ) -> Vec<PullRequestCommitPayload> {
        self.client
            .get(&format!(
                "https://api.github.com/repos/{}/pulls/{}/commits",
                repository_name, pull_request_number
            ))
            .send()
            .unwrap()
            .json()
            .unwrap()
    }

    pub fn list_pull_requests(&self, repository_name: &str) -> Vec<PullRequestPayload> {
        self.client
            .get(&format!(
                "https://api.github.com/repos/{}/pulls",
                repository_name
            ))
            .send()
            .unwrap()
            .json()
            .unwrap()
    }

    pub fn approve_pull_requests(
        &self,
        repository_name: &str,
        pull_request_number: u64,
    ) -> ResultReqwest {
        self.client
            .post(&format!(
                "https://api.github.com/repos/{}/pulls/{}/reviews",
                repository_name, pull_request_number,
            ))
            .body(
                json!({
                    "event": "APPROVE",
                })
                .to_string(),
            )
            .send()
    }

    pub fn branch_info(&self, repository_name: &str, branch: &str) -> BranchPayload {
        self.client
            .get(&format!(
                "https://api.github.com/repos/{}/branches/{}",
                repository_name, branch,
            ))
            .send()
            .unwrap()
            .json()
            .unwrap()
    }

    pub fn search_pull_requests(&self, repository_name: &str, term: &str) -> SearchPayload {
        self.client
            .get(&format!(
                "https://api.github.com/search/issues?q={}+is:pr+repo:{}",
                term, repository_name,
            ))
            .send()
            .unwrap()
            .json()
            .unwrap()
    }
}
