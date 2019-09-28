use super::Command;
use github_client::github::GithubClient;
use helpable_derive::ChoosablePullRequest;

#[derive(Debug, Default, StructOpt, ChoosablePullRequest)]
#[structopt(rename_all = "kebab-case")]
pub struct Approve {
    #[structopt(short = "pr", long = "pull_request_number")]
    /// Number of the Pull Request to update
    pub pull_request_number: Option<u64>,
}

impl Command for Approve {
    fn execute(&self, github_client: GithubClient) -> Result<(), String> {
        let pull_request_number: u64 =
            Self::pull_request_number(self.pull_request_number, &github_client)?;

        github_client
            .approve_pull_requests("", pull_request_number)
            .unwrap();

        Ok(())
    }
}
