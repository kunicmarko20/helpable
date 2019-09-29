#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate text_io;

use crate::config::Config;
use crate::helpable::HelpableSubCommand;
use crate::helpable::{ConfigSubCommand, Helpable};
use crate::structopt::StructOpt;
use github_client::github::GithubClient;

mod command;
mod config;
mod helpable;

fn main() {
    let mut config = Config::new();
    let github_client = GithubClient::new(config.get("github_access_token"));

    let repository_name = config.get("repository_name");
    let ticket_prefix = config.get("ticket_prefix");

    let result = match Helpable::from_args().command {
        HelpableSubCommand::Release(command) => {
            command.execute(github_client, &repository_name, &ticket_prefix)
        }
        HelpableSubCommand::UpdateRelease(command) => {
            command.execute(github_client, &repository_name, &ticket_prefix)
        }
        HelpableSubCommand::Approve(command) => command.execute(github_client, &repository_name),
        HelpableSubCommand::NewestCommitSha(command) => {
            command.execute(github_client, &repository_name)
        }
        HelpableSubCommand::Merge(command) => command.execute(github_client, &repository_name),
        HelpableSubCommand::Config { command } => match command {
            ConfigSubCommand::List(command) => command.execute(config),
            ConfigSubCommand::Set(mut command) => command.execute(config),
        },
    };

    if let Err(message) = result {
        println!("{}", message);
    }
}
