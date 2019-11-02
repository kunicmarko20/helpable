use crate::config::Config;
use github_client::github::GithubClient;

#[derive(Debug, Default, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct NewestCommitSha {
    /// Branch to fetch the latest sha from
    pub branch: Option<String>,
}

impl NewestCommitSha {
    pub fn execute(&self, github_client: GithubClient, mut config: Config) -> Result<(), String> {
        let response = github_client
            .branch_info(
                &config.get("repository_name"),
                &self
                    .branch
                    .clone()
                    .unwrap_or_else(|| config.get("release_branch")),
            )
            .map_err(|error| error.to_string())?;

        println!("{}", response.sha());

        Ok(())
    }
}
