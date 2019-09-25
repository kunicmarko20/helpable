use dialoguer::{theme::ColorfulTheme, Select};
use github_client::github::GithubClient;
use github_client::github::MergeMethod;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Merge {
    #[structopt(short = "pr", long = "pull_request_number")]
    /// Number of the Pull Request to update
    pub pull_request_number: Option<u64>,
}

impl Merge {
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
            println!("Unable to merge, did you try to approve pull request?");
        }
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
