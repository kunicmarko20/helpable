use super::ChoosablePullRequest;
use super::Command;
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

impl ChoosablePullRequest for UpdateRelease {}

impl UpdateRelease {
    pub fn new(pull_request_number: u64) -> Self {
        UpdateRelease {
            pull_request_number: Some(pull_request_number),
        }
    }
}

impl Command for UpdateRelease {
    fn execute(&self, github_client: GithubClient) -> Result<(), String> {
        let pull_request_number: u64 =
            Self::pull_request_number(self.pull_request_number, &github_client)?;

        let pull_request = github_client.pull_request_info("", pull_request_number);

        if !pull_request.is_release() {
            return Err("Not a release Pull Request.".to_string());
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
            return Err("Nothing to update.".to_string());
        }

        let body = json!({
            "title": title.as_str(),
        });

        github_client
            .update_pull_request("", pull_request_number, body.to_string())
            .unwrap();

        Ok(())
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
