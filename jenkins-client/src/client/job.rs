use super::JenkinsConfig;
use super::Result as JenkinsResult;
use crate::payload::build::Build;
use reqwest::Client;
use std::collections::HashMap;

pub struct JobClient<'a> {
    pub(super) client: &'a Client,
    pub(super) config: &'a JenkinsConfig,
}

impl<'a> JobClient<'a> {
    pub fn last_build<T: Into<JobName>>(&self, job_name: T) -> JenkinsResult<Build> {
        let mut response = self
            .client
            .get(&format!(
                "{}/job/{}/lastBuild/api/json",
                &self.config.url,  job_name.into().0,
            ))
            .basic_auth(&self.config.username, self.config.token.clone())
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err("Unable to fetch job, did you enable vpn?".to_string());
        }

        Ok(response.json().unwrap())
    }

    pub fn build_with_parameters<T>(
        &self,
        job_name: T,
        parameters: HashMap<String, String>,
    ) -> JenkinsResult<()>
        where T: Into<JobName>
    {
        let mut response = self
            .client
            .post(&format!(
                "{}/job/{}/buildWithParameters?{}",
                &self.config.url,
                job_name.into().0,
                serde_urlencoded::to_string(parameters).map_err(|error| error.to_string())?
            ))
            .basic_auth(&self.config.username, self.config.token.clone())
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(())
    }

    pub fn build<T: Into<JobName>>(&self, job_name: T) -> JenkinsResult<()> {
        let mut response = self
            .client
            .post(&format!("{}/job/{}/build", &self.config.url, job_name.into().0))
            .basic_auth(&self.config.username, self.config.token.clone())
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(())
    }
}

pub struct JobName(String);

impl From<String> for JobName {
    fn from(s: String) -> Self {
        JobName(
            s.trim_start_matches("job/")
                .trim_start_matches("/job/")
                .trim_start_matches('/')
                .trim_end_matches('/')
                .to_owned()
        )
    }
}
