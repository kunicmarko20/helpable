use super::Command;
use github_client::github::GithubClient;
use github_client::github::MergeMethod;
use helpable_derive::ChoosablePullRequest;
use structopt::StructOpt;

#[derive(Debug, StructOpt, ChoosablePullRequest)]
#[structopt(rename_all = "kebab-case")]
pub struct Merge {
    #[structopt(short = "pr", long = "pull_request_number")]
    /// Number of the Pull Request to update
    pub pull_request_number: Option<u64>,
}

impl Command for Merge {
    fn execute(&self, github_client: GithubClient) -> Result<(), String> {
        let pull_request_number: u64 =
            Self::pull_request_number(self.pull_request_number, &github_client)?;

        let pull_request = github_client.pull_request_info("", pull_request_number);

        let body = if pull_request.is_release() || pull_request.is_back_merge() {
            json!({
                "merge_method": Into::<&str>::into(MergeMethod::Merge),
            })
        } else {
            json!({
                "commit_title": pull_request.title(),
                "merge_method": Into::<&str>::into(MergeMethod::Squash),
            })
        };

        let response = github_client
            .merge_pull_request("", pull_request_number, body.to_string())
            .unwrap();

        if response.status() == 405 {
            return Err("Unable to merge, did you try to approve pull request?".to_string());
        }

        Ok(())
    }
}
