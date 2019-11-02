use crate::config::Config;
use github_client::github::GithubClient;
use helpable_derive::ChoosablePullRequest;

#[derive(Debug, Default, StructOpt, ChoosablePullRequest)]
#[structopt(rename_all = "kebab-case")]
pub struct Approve {
    /// Number of the pull request to update
    pub pull_request_number: Option<u64>,
}

impl Approve {
    pub fn execute(&self, github_client: GithubClient, mut config: Config) -> Result<(), String> {
        let repository_name = config.get("repository_name");

        let pull_request_number: u64 =
            Self::pull_request_number(self.pull_request_number, &github_client, &repository_name)?;

        github_client
            .pull_request()
            .approve(&repository_name, pull_request_number)
            .map_err(|error| error.to_string())?;

        Ok(())
    }
}
