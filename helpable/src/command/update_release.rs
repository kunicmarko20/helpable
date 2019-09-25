use dialoguer::{theme::ColorfulTheme, Select};
use github_client::github::GithubClient;
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"\[(?P<ticket>(CARD|LOAN)-\d+)\].*").unwrap();
}

#[derive(Debug, Default, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct UpdateRelease {
    #[structopt(short = "pr", long = "pull_request_number")]
    /// Number of the Pull Request to update
    pub pull_request_number: Option<u64>,
}

impl UpdateRelease {
    pub fn new(pull_request_number: u64) -> Self {
        UpdateRelease {
            pull_request_number: Some(pull_request_number),
        }
    }

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

        if !pull_request.is_release() {
            return;
        }

        let mut title = "Release".to_string();

        let pull_request_commits = github_client.pull_request_commits("", pull_request_number);

        for pull_request_commit in pull_request_commits {
            if let Some(captures) = REGEX.captures(&pull_request_commit.commit_message()) {
                if let Some(ticket) = captures.name("ticket") {
                    title = title + " " + ticket.as_str();
                }
            }
        }

        if pull_request.title() == title {
            return;
        }

        let body = json!({
            "title": title.as_str(),
        });

        github_client
            .update_pull_request("", pull_request_number, body.to_string())
            .unwrap();
    }

    fn choose_pull_request(github_client: &GithubClient) -> Result<u64, &str> {
        let pull_requests = github_client.list_pull_requests("");

        let selections: Vec<&str> = pull_requests
            .iter()
            .filter(|pull_request| pull_request.title().contains("Release"))
            .map(|pull_request| pull_request.title())
            .collect();

        if selections.is_empty() {
            return Err("No Release Pull Requests found in repository.");
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

#[cfg(test)]
mod tests {
    use super::REGEX;

    #[test]
    fn it_will_match_regex() {
        let captures = REGEX.captures("[CARD-321] this is a pr").unwrap();

        assert!(captures.name("ticket").unwrap().as_str() == "CARD-321");

        let captures = REGEX.captures("[LOAN-321] this is a pr").unwrap();

        assert!(captures.name("ticket").unwrap().as_str() == "LOAN-321");
    }
}
