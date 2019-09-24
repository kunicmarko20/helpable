use super::command::{Release, UpdateRelease, ApprovePullRequest};

#[derive(Debug, StructOpt)]
pub struct Helpable {
    #[structopt(subcommand)]
    pub command: HelpableSubCommand
}

#[derive(Debug, StructOpt)]
pub enum HelpableSubCommand {
    #[structopt(name = "release")]
    /// Creates a Release PR
    Release(Release),
    #[structopt(name = "update-release")]
    /// Updates Release PR name
    UpdateRelease(UpdateRelease),
    #[structopt(name = "approve")]
    /// Approve PR
    ApprovePullRequest(ApprovePullRequest),
}
