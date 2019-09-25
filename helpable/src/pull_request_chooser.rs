use dialoguer::{theme::ColorfulTheme, Select};
use github_client::github::payload::PullRequestPayload;
use github_client::github::GithubClient;

pub struct PullRequestChooser;

impl PullRequestChooser {
    pub fn choose(github_client: &GithubClient) -> Result<PullRequestPayload, String> {
        let pull_requests = github_client.list_pull_requests("");

        let selections: Vec<&str> = pull_requests
            .iter()
            .map(|pull_request| pull_request.title())
            .collect();

        if selections.is_empty() {
            return Err("No Release Pull Requests found in repository.".to_string());
        }

        let selected = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose Pull Request:")
            .items(&selections[..])
            .interact()
            .unwrap();

        let selected_title = selections[selected].to_string();

        Ok(pull_requests
            .into_iter()
            .find(|pull_request| pull_request.title() == selected_title)
            .expect("Unable to find matching Pull Request.")
        )
    }
}
