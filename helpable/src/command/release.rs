use super::Command;
use crate::command::UpdateRelease;
use github_client::github::GithubClient;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Release {}

impl Command for Release {
    fn execute(&self, github_client: GithubClient, repository_name: &str) -> Result<(), String> {
        let body = json!({
            "title": "Release",
            "head": "development",
            "base": "master",
            "maintainer_can_modify": true,
        });

        let response = github_client
            .create_pull_request(repository_name, body.to_string())
            .unwrap();

        UpdateRelease::new(response.pull_request_number())
            .execute(github_client, repository_name)?;

        Ok(())
    }
}
