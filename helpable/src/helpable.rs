use super::command::config::{List, Set};
use super::command::{Approve, Merge, NewestCommitSha, Release, Search, UpdateRelease};

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
    /// Approve a pull request
    Approve(Approve),
    #[structopt(name = "sha")]
    /// Get newest Commit Sha
    NewestCommitSha(NewestCommitSha),
    /// Merge a pull request
    Merge(Merge),
    /// Search pull requests by term and open chosen one in default browser
    Search(Search),
    /// Set or view config values
    Config {
        #[structopt(subcommand)]
        command: ConfigSubCommand,
    },
}

#[derive(Debug, StructOpt)]
pub enum ConfigSubCommand {
    /// Show all current config values
    List(List),
    /// Set or replace a config value
    Set(Set),
}
