use crate::config::Config;
use github_client::github::payload::PullRequestPayload;
use github_client::github::GithubClient;
use github_client::github::MergeMethod;
use helpable_derive::{ChoosablePullRequest, Confirmation};
use structopt::StructOpt;

#[derive(Debug, StructOpt, ChoosablePullRequest, Confirmation)]
#[structopt(rename_all = "kebab-case")]
pub struct Merge {
    /// Number of the pull request to update
    pub pull_request_number: Option<u64>,
}

impl Merge {
    pub fn execute(&self, github_client: GithubClient, mut config: Config) -> Result<(), String> {
        let repository_name = config.get("repository_name");

        let pull_request_number: u64 =
            Self::pull_request_number(self.pull_request_number, &github_client, &repository_name)?;

        let pull_request = github_client
            .pull_request()
            .get(&repository_name, pull_request_number)
            .map_err(|error| error.to_string())?;

        let body = if Self::should_create_merge_commit(&mut config, &pull_request) {
            json!({
                "merge_method": Into::<&str>::into(MergeMethod::Merge),
            })
        } else {
            json!({
                "commit_title": pull_request.title().to_string() + &format!(" (#{})", pull_request_number),
                "merge_method": Into::<&str>::into(MergeMethod::Squash),
            })
        };

        Self::confirmation(&format!(
            "You are about to merge \"{}\", are you sure?",
            pull_request.title().to_string()
        ))?;

        let response = github_client.pull_request().merge(
            &repository_name,
            pull_request_number,
            body.to_string(),
        );

        if response.is_err() {
            webbrowser::open(pull_request.html_url()).unwrap();
            response.map_err(|error| error.to_string())?;
        }

        Ok(())
    }

    fn should_create_merge_commit(config: &mut Config, pull_request: &PullRequestPayload) -> bool {
        pull_request.is_release(
            config.get("release_branch"),
            config.get("development_branch"),
        ) || pull_request.is_back_merge(
            config.get("release_branch"),
            config.get("development_branch"),
        )
    }
}
