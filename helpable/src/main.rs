#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate text_io;

use crate::config::Config;
use crate::helpable::{ConfigSubCommand, Helpable};
use crate::helpable::{HelpableSubCommand, JenkinsSubCommand};
use crate::structopt::StructOpt;
use github_client::GithubClient;
use jenkins_client::{JenkinsClient, JenkinsConfig};

mod command;
mod config;
mod helpable;

fn main() {
    let mut config = Config::new();
    let github_client = GithubClient::new(config.get("github_access_token"));

    let result = match Helpable::from_args().command {
        HelpableSubCommand::Release(command) => command.execute(github_client, config),
        HelpableSubCommand::UpdateRelease(command) => command.execute(github_client, config),
        HelpableSubCommand::Approve(command) => command.execute(github_client, config),
        HelpableSubCommand::NewestCommitSha(command) => command.execute(github_client, config),
        HelpableSubCommand::Merge(command) => command.execute(github_client, config),
        HelpableSubCommand::Search(command) => command.execute(github_client, config),
        HelpableSubCommand::Config { command } => match command {
            ConfigSubCommand::List(command) => command.execute(config),
            ConfigSubCommand::Set(mut command) => command.execute(config),
        },
        HelpableSubCommand::Jenkins { command } => {
            let jenkins = jenkins_client_from_config(&mut config);
            match command {
                JenkinsSubCommand::Release(command) => {
                    command.execute(github_client, config, jenkins)
                }
            }
        }
    };

    if let Err(message) = result {
        println!("{}", message);
    }
}

pub fn jenkins_client_from_config(config: &mut Config) -> JenkinsClient {
    JenkinsClient::new(JenkinsConfig::new(
        config.get("jenkins_url"),
        config.get("jenkins_username"),
        config.get("jenkins_access_token"),
    ))
}
