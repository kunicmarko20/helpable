use crate::command::UpdateRelease;
use crate::config::Config;
use github_client::github::GithubClient;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Release {}

impl Release {
    pub fn execute(&self, github_client: GithubClient, mut config: Config) -> Result<(), String> {
        let body = json!({
            "title": "Release",
            "head": config.get("development_branch"),
            "base": config.get("release_branch"),
            "maintainer_can_modify": true,
        });

        let response = github_client
            .create_pull_request(&config.get("repository_name"), body.to_string())
            .unwrap();

        UpdateRelease::new(response.pull_request_number()).execute(github_client, config)?;

        Ok(())
    }
}
