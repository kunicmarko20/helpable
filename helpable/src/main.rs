#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

use crate::command::Command;
use crate::helpable::{Helpable, ConfigSubCommand};
use crate::helpable::HelpableSubCommand;
use crate::structopt::StructOpt;
use github_client::github::GithubClient;

mod command;
mod helpable;

fn main() {
    let github_client = GithubClient::default();

    let result = match Helpable::from_args().command {
        HelpableSubCommand::Release(command) => command.execute(github_client),
        HelpableSubCommand::UpdateRelease(command) => command.execute(github_client),
        HelpableSubCommand::Approve(command) => command.execute(github_client),
        HelpableSubCommand::NewestCommitSha(command) => command.execute(github_client),
        HelpableSubCommand::Merge(command) => command.execute(github_client),
        HelpableSubCommand::Config{ command} => match command {
            ConfigSubCommand::List(command) => command.execute(),
            ConfigSubCommand::Set(command) => command.execute(),
        },
    };

    if let Err(message) = result {
        println!("{}", message);
    }
}
