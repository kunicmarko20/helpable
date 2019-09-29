use super::Command;
use github_client::github::GithubClient;

#[derive(Debug, Default, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct NewestCommitSha {}

impl Command for NewestCommitSha {
    fn execute(&self, github_client: GithubClient, repository_name: &str) -> Result<(), String> {
        let response = github_client.branch_info(repository_name, "master");

        println!("{}", response.sha());

        Ok(())
    }
}
