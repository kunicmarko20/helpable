use github_client::github::GithubClient;

#[derive(Debug, Default, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct NewestCommitSha {}

impl NewestCommitSha {
    pub fn execute(&self, github_client: GithubClient) {
        let response = github_client.branch_info("", "master");

        println!("{}", response.sha());
    }
}
