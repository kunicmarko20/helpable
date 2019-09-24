use github_client::github::GithubClient;
use dialoguer::{theme::ColorfulTheme, Select};

#[derive(Debug, Default, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct ApprovePullRequest {
    #[structopt(short = "pr", long = "pull_request_number")]
    /// Number of the Pull Request to update
    pub pull_request_number: Option<u64>,
}

impl ApprovePullRequest {
    pub fn execute(&self, github_client: GithubClient) {
        let pull_request_number: u64 = self.pull_request_number.unwrap_or_else(|| {
            let choice = Self::choose_pull_request(&github_client);

            if let Err(message) = choice {
                println!("{}", message);
                return;
            }

            return choice.unwrap();
        });

        github_client
            .approve_pull_requests(
                "",
                pull_request_number,
            )
            .unwrap();
    }

    fn choose_pull_request(github_client: &GithubClient) -> Result<u64, &str> {
        let pull_requests = github_client.list_pull_requests("");

        let selections: Vec<&str> = pull_requests.iter()
            .map(|pull_request| pull_request.title())
            .collect();

        if selections.is_empty() {
            return Err("No Pull Requests found in repository.");
        }

        let selected = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose Pull Request:")
            .items(&selections[..])
            .interact()
            .unwrap();

        let selected_title = selections[selected];

        Ok(pull_requests
            .iter()
            .find(|pull_request| pull_request.title() == selected_title)
            .expect("Unable to find matching Pull Request.")
            .pull_request_number())
    }
}
