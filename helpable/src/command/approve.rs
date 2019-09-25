use dialoguer::{theme::ColorfulTheme, Select};
use github_client::github::GithubClient;

#[derive(Debug, Default, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Approve {
    #[structopt(short = "pr", long = "pull_request_number")]
    /// Number of the Pull Request to update
    pub pull_request_number: Option<u64>,
}

impl Approve {
    pub fn execute(&self, github_client: GithubClient) {
        let mut pull_request_number: Option<u64> = self.pull_request_number;

        if pull_request_number.is_none() {
            let choice = Self::choose_pull_request(&github_client);

            if let Err(message) = choice {
                println!("{}", message);
                return;
            }

            pull_request_number = Some(choice.unwrap());
        }

        let pull_request_number = pull_request_number.unwrap();

        github_client
            .approve_pull_requests("", pull_request_number)
            .unwrap();
    }

    fn choose_pull_request(github_client: &GithubClient) -> Result<u64, &str> {
        let pull_requests = github_client.list_pull_requests("");

        let selections: Vec<&str> = pull_requests
            .iter()
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
