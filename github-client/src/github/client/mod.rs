use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, Response,
};

use crate::github::payload::ErrorPayload;
use branch::BranchClient;
use comment::CommentClient;
use pull_request::PullRequestClient;

mod branch;
mod comment;
mod pull_request;

pub type Result<T> = ::std::result::Result<T, ErrorPayload>;
pub type GenericResponse = Result<Response>;

const BASE_URL: &str = "https://api.github.com/";

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

    pub fn pull_request(&self) -> PullRequestClient {
        PullRequestClient {
            client: &self.client,
        }
    }

    pub fn branch(&self) -> BranchClient {
        BranchClient {
            client: &self.client,
        }
    }

    pub fn comment(&self) -> CommentClient {
        CommentClient {
            client: &self.client,
        }
    }
}
