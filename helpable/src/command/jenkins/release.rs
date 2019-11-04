use crate::config::Config;
use github_client::GithubClient;
use jenkins_client::JenkinsClient;
use std::collections::HashMap;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Release {}

impl Release {
    pub fn execute(
        &self,
        github_client: GithubClient,
        mut config: Config,
        jenkins: JenkinsClient,
    ) -> Result<(), String> {
        let last_build = jenkins
            .job()
            .last_build(&config.get("jenkins_build_check"))?;

        if !last_build.is_success() {
            return Err(last_build.to_string());
        }

        let response = github_client
            .branch()
            .get(
                &config.get("repository_name"),
                &config.get("release_branch"),
            )
            .map_err(|error| error.to_string())?;

        let mut parameters: HashMap<String, String> = HashMap::new();
        parameters.insert("TARGET_COMMIT".to_string(), response.sha().to_owned());

        jenkins
            .job()
            .build_with_parameters(&config.get("jenkins_build_deploy"), parameters)?;

        Ok(())
    }
}
