use super::command::{Approve, Merge, NewestCommitSha, Release, UpdateRelease};
use super::command::config::{List, Set};

#[derive(Debug, StructOpt)]
pub struct Helpable {
    #[structopt(subcommand)]
    pub command: HelpableSubCommand,
}

#[derive(Debug, StructOpt)]
pub enum HelpableSubCommand {
    /// Creates a Release PR
    Release(Release),
    /// Updates Release PR name
    UpdateRelease(UpdateRelease),
    /// Approve PR
    Approve(Approve),
    #[structopt(name = "sha")]
    /// Newest Commit Sha
    NewestCommitSha(NewestCommitSha),
    /// Merge Pull Request
    Merge(Merge),
    /// Config
    Config {
        #[structopt(subcommand)]
        command: ConfigSubCommand,
    }
}

#[derive(Debug, StructOpt)]
pub enum ConfigSubCommand {
    /// Lists config values
    List(List),
    /// Set new config value
    Set(Set),
}
