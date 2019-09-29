use super::command::config::{List, Set};
use super::command::{Approve, Merge, NewestCommitSha, Release, UpdateRelease};

#[derive(Debug, StructOpt)]
pub struct Helpable {
    #[structopt(subcommand)]
    pub command: HelpableSubCommand,
}

#[derive(Debug, StructOpt)]
pub enum HelpableSubCommand {
    /// Create a Release pull request
    Release(Release),
    /// Update Release pull request name
    UpdateRelease(UpdateRelease),
    /// Approve pull request
    Approve(Approve),
    #[structopt(name = "sha")]
    /// Newest Commit Sha
    NewestCommitSha(NewestCommitSha),
    /// Merge pull request
    Merge(Merge),
    /// Set or view config values
    Config {
        #[structopt(subcommand)]
        command: ConfigSubCommand,
    },
}

#[derive(Debug, StructOpt)]
pub enum ConfigSubCommand {
    /// Lists config values
    List(List),
    /// Set new config value
    Set(Set),
}
