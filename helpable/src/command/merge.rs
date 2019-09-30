use github_client::github::GithubClient;
use github_client::github::MergeMethod;
use helpable_derive::ChoosablePullRequest;
use structopt::StructOpt;

#[derive(Debug, StructOpt, ChoosablePullRequest)]
#[structopt(rename_all = "kebab-case")]
pub struct Merge {
    /// Number of the Pull Request to update
    pub pull_request_number: Option<u64>,
}

impl Merge {
    pub fn execute(
        &self,
        github_client: GithubClient,
        repository_name: &str,
    ) -> Result<(), String> {
        let pull_request_number: u64 =
            Self::pull_request_number(self.pull_request_number, &github_client, repository_name)?;

        let pull_request = github_client.pull_request_info(repository_name, pull_request_number);

        let body = if pull_request.is_release() || pull_request.is_back_merge() {
            json!({
                "merge_method": Into::<&str>::into(MergeMethod::Merge),
            })
        } else {
            json!({
                "commit_title": pull_request.title().to_string() + &format!(" (#{})", pull_request_number),
                "merge_method": Into::<&str>::into(MergeMethod::Squash),
            })
        };

        let response = github_client
            .merge_pull_request(repository_name, pull_request_number, body.to_string())
            .unwrap();

        if response.status() == 405 {
            return Err("Unable to merge, did you try to approve pull request?".to_string());
        }

        Ok(())
    }
}
