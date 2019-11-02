use crate::config::Config;
use github_client::github::GithubClient;
use helpable_derive::ChoosablePullRequest;
use regex::Regex;

#[derive(Debug, Default, StructOpt, ChoosablePullRequest)]
#[structopt(rename_all = "kebab-case")]
pub struct UpdateRelease {
    /// Number of the pull request to update
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
    pub fn execute(&self, github_client: GithubClient, mut config: Config) -> Result<(), String> {
        let repository_name = config.get("repository_name");

        let pull_request_number: u64 =
            Self::pull_request_number(self.pull_request_number, &github_client, &repository_name)?;

        let pull_request = github_client
            .pull_request()
            .get(&repository_name, pull_request_number)
            .map_err(|error| error.to_string())?;

        if !pull_request.is_release(
            config.get("release_branch"),
            config.get("development_branch"),
        ) {
            return Err("Not a release Pull Request.".to_string());
        }

        let tickets = Self::tickets_from_commits(
            &github_client,
            &repository_name,
            pull_request_number,
            config.get("ticket_prefix"),
        )?;

        let new_title = "Release ".to_owned() + &tickets.join(" ");

        if pull_request.title() == new_title {
            return Err("Nothing to update.".to_string());
        }

        let pull_request_body = tickets
            .into_iter()
            .map(|ticket| config.get("ticket_url") + &ticket)
            .collect::<Vec<String>>()
            .join("\n");

        let body = json!({
            "title": new_title,
            "body": pull_request_body,
        });

        github_client
            .pull_request()
            .update(&repository_name, pull_request_number, body.to_string())
            .map_err(|error| error.to_string())?;

        Ok(())
    }

    fn tickets_from_commits(
        github_client: &GithubClient,
        repository_name: &str,
        pull_request_number: u64,
        ticket_prefix: String,
    ) -> Result<Vec<String>, String> {
        let mut tickets = Vec::new();

        let pull_request_commits = github_client
            .pull_request()
            .commits(&repository_name, pull_request_number)
            .map_err(|error| error.to_string())?;

        let regex = Regex::new(&format!("\\[(?P<ticket>{}-\\d+)\\].*", ticket_prefix)).unwrap();

        for pull_request_commit in pull_request_commits {
            if let Some(captures) = regex.captures(&pull_request_commit.commit_message()) {
                if let Some(ticket) = captures.name("ticket") {
                    let ticket_string = ticket.as_str().to_owned();

                    if !tickets.contains(&ticket_string) {
                        tickets.push(ticket_string);
                    }
                }
            }
        }

        Ok(tickets)
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
