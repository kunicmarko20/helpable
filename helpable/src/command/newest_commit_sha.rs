use super::Command;
use github_client::github::GithubClient;

#[derive(Debug, Default, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct NewestCommitSha {}

impl Command for NewestCommitSha {
    fn execute(&self, github_client: GithubClient) -> Result<(), String> {
        let response = github_client.branch_info("", "master");

        println!("{}", response.sha());

        Ok(())
    }
}
