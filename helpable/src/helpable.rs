use super::command::Release;

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
}
