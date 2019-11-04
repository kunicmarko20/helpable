use reqwest::Client;

pub use self::job::JobClient;

mod job;

pub type Result<T> = ::std::result::Result<T, String>;

pub struct JenkinsClient {
    client: Client,
    config: JenkinsConfig,
}

impl JenkinsClient {
    pub fn new(config: JenkinsConfig) -> Self {
        let client = Client::builder().build().unwrap();

        JenkinsClient { client, config }
    }

    pub fn job(&self) -> JobClient {
        JobClient {
            client: &self.client,
            config: &self.config,
        }
    }
}

pub struct JenkinsConfig {
    url: String,
    username: String,
    token: Option<String>,
}

impl JenkinsConfig {
    pub fn new(url: String, username: String, token: String) -> Self {
        JenkinsConfig {
            url: url.trim_end_matches('/').to_owned(),
            username,
            token: Some(token),
        }
    }
}
