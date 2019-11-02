use crate::config::Config;
use github_client::github::GithubClient;
use helpable_derive::ChoosablePullRequest;

#[derive(Debug, Default, StructOpt, ChoosablePullRequest)]
#[structopt(rename_all = "kebab-case")]
pub struct Search {
    /// Term to search for
    pub term: String,
}

impl Search {
    pub fn execute(&self, github_client: GithubClient, mut config: Config) -> Result<(), String> {
        let repository_name = config.get("repository_name");

        let search_payload = github_client
            .search_pull_requests(&repository_name, &self.term)
            .map_err(|error| error.to_string())?;

        let chosen = Self::choose(
            search_payload
                .items()
                .iter()
                .map(|pull_request| pull_request.title())
                .collect(),
            &repository_name,
        )?;

        webbrowser::open(
            search_payload
                .items()
                .iter()
                .find(|pull_request| pull_request.title() == chosen)
                .expect("Unable to find matching Pull Request.")
                .html_url(),
        )
        .unwrap();

        Ok(())
    }
}
