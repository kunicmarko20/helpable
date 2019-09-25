use super::command::{Release, UpdateRelease, Approve};

#[derive(Debug, StructOpt)]
pub struct Helpable {
    #[structopt(subcommand)]
    pub command: HelpableSubCommand
}

#[derive(Debug, StructOpt)]
pub enum HelpableSubCommand {
    /// Creates a Release PR
    Release(Release),
    /// Updates Release PR name
    UpdateRelease(UpdateRelease),
    /// Approve PR
    Approve(Approve),
}
