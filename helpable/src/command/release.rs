use structopt::StructOpt;
use github_client::github::GithubClient;
use crate::command::UpdateRelease;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Release {}

impl Release {
    pub fn execute(&self, github_client: GithubClient) {
        let body = json!({
            "title": "Release",
            "head": "development",
            "base": "master",
            "maintainer_can_modify": true,
        });

        let response = github_client.create_pull_request(
            "",
            body.to_string(),
        ).unwrap();

        UpdateRelease::new(response.pull_request_number()).execute(github_client);
    }
}
