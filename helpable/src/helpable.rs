use super::command::{Release, UpdateRelease};

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
    /// Updates Release PR name
    UpdateRelease(UpdateRelease),
}
