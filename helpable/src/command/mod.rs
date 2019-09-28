pub mod approve;
pub mod merge;
pub mod newest_commit_sha;
pub mod release;
pub mod update_release;

pub use approve::Approve;
use github_client::github::GithubClient;
pub use merge::Merge;
pub use newest_commit_sha::NewestCommitSha;
pub use release::Release;
pub use update_release::UpdateRelease;

pub trait Command {
    fn execute(&self, github_client: GithubClient) -> Result<(), String>;
}
