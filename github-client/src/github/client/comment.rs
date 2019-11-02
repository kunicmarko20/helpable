use super::{GenericResponse, BASE_URL};
use reqwest::Client;

pub struct CommentClient<'a> {
    pub(super) client: &'a Client,
}

impl<'a> CommentClient<'a> {
    pub fn create(
        &self,
        repository_name: &str,
        issue_number: u64,
        body: String,
    ) -> GenericResponse {
        let mut response = self
            .client
            .post(&format!(
                "{}repos/{}/issues/{}/comments",
                BASE_URL, repository_name, issue_number,
            ))
            .body(body)
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(response)
    }
}
