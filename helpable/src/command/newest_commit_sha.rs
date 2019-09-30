use github_client::github::GithubClient;

#[derive(Debug, Default, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct NewestCommitSha {
    /// Branch to fetch the latest sha from
    pub branch: Option<String>,
}

impl NewestCommitSha {
    pub fn execute(
        &self,
        github_client: GithubClient,
        repository_name: &str,
    ) -> Result<(), String> {
        let response = github_client.branch_info(
            repository_name,
            &self.branch.clone().unwrap_or_else(|| "master".to_string()),
        );

        println!("{}", response.sha());

        Ok(())
    }
}
