use super::JenkinsConfig;
use super::Result;
use crate::payload::build::Build;
use reqwest::Client;
use std::collections::HashMap;

pub struct JobClient<'a> {
    pub(super) client: &'a Client,
    pub(super) config: &'a JenkinsConfig,
}

impl<'a> JobClient<'a> {
    pub fn last_build(&self, job_name: &str) -> Result<Build> {
        let mut response = self
            .client
            .get(&format!(
                "{}/job/{}/lastBuild/api/json",
                &self.config.url, job_name,
            ))
            .basic_auth(&self.config.username, self.config.token.clone())
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err("Unable to fetch job, did you enable vpn?".to_string());
        }

        Ok(response.json().unwrap())
    }

    pub fn build_with_parameters(
        &self,
        job_name: &str,
        parameters: HashMap<String, String>,
    ) -> Result<()> {
        let mut response = self
            .client
            .post(&format!(
                "{}/job/{}/buildWithParameters?{}",
                &self.config.url,
                job_name,
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

    pub fn build(&self, job_name: &str) -> Result<()> {
        let mut response = self
            .client
            .post(&format!("{}/job/{}/build", &self.config.url, job_name))
            .basic_auth(&self.config.username, self.config.token.clone())
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response.json().unwrap());
        }

        Ok(())
    }
}
