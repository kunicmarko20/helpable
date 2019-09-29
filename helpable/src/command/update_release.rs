use github_client::github::GithubClient;
use helpable_derive::ChoosablePullRequest;
use regex::Regex;

#[derive(Debug, Default, StructOpt, ChoosablePullRequest)]
#[structopt(rename_all = "kebab-case")]
pub struct UpdateRelease {
    /// Number of the Pull Request to update
    pub pull_request_number: Option<u64>,
}

impl UpdateRelease {
    pub fn new(pull_request_number: u64) -> Self {
        UpdateRelease {
            pull_request_number: Some(pull_request_number),
        }
    }
}

impl UpdateRelease {
    pub fn execute(
        &self,
        github_client: GithubClient,
        repository_name: &str,
        ticket_prefix: &str,
    ) -> Result<(), String> {
        let pull_request_number: u64 =
            Self::pull_request_number(self.pull_request_number, &github_client, repository_name)?;
        let pull_request = github_client.pull_request_info(repository_name, pull_request_number);

        if !pull_request.is_release() {
            return Err("Not a release Pull Request.".to_string());
        }

        let mut title = "Release".to_string();

        let pull_request_commits =
            github_client.pull_request_commits(repository_name, pull_request_number);

        let regex = Regex::new(&format!("\\[(?P<ticket>{}-\\d+)\\].*", ticket_prefix)).unwrap();

        for pull_request_commit in pull_request_commits {
            if let Some(captures) = regex.captures(&pull_request_commit.commit_message()) {
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
            .update_pull_request(repository_name, pull_request_number, body.to_string())
            .unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn it_will_match_regex() {
        let regex = Regex::new(&format!("\\[(?P<ticket>{}-\\d+)\\].*", "(TEST|PROJECT)")).unwrap();

        let captures = regex.captures("[TEST-321] this is a pr").unwrap();

        assert!(captures.name("ticket").unwrap().as_str() == "TEST-321");

        let captures = regex.captures("[PROJECT-321] this is a pr").unwrap();

        assert!(captures.name("ticket").unwrap().as_str() == "PROJECT-321");
    }
}
