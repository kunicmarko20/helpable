#[derive(Deserialize, Debug)]
pub struct BranchCommit {
    pub(in crate::github) sha: String,
}
