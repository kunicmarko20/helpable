use crate::pull_request_chooser::PullRequestChooser;
use github_client::github::GithubClient;

pub trait ChoosablePullRequest {
    fn pull_request_number(
        pull_request_number: Option<u64>,
        github_client: &GithubClient,
    ) -> Result<u64, String> {
        if pull_request_number.is_none() {
            return PullRequestChooser::choose(&github_client)
                .and_then(|pull_request| Ok(pull_request.pull_request_number()));
        }

        Ok(pull_request_number.unwrap())
    }
}
