use super::parts::BranchCommit;

#[derive(Deserialize, Debug)]
pub struct BranchPayload {
    commit: BranchCommit,
}

impl BranchPayload {
    pub fn sha(&self) -> &str {
        &self.commit.sha
    }
}
