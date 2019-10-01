use super::parts::{Base, Head};

#[derive(Deserialize, Debug)]
pub struct PullRequestPayload {
    number: u64,
    title: String,
    base: Base,
    head: Head,
}

impl PullRequestPayload {
    pub fn is_release(&self, release_branch: String, development_branch: String) -> bool {
        self.base.branch == release_branch && self.head.branch == development_branch
    }

    pub fn is_back_merge(&self, release_branch: String, development_branch: String) -> bool {
        self.base.branch == release_branch && self.head.branch == development_branch
    }

    pub fn pull_request_number(&self) -> u64 {
        self.number
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}
