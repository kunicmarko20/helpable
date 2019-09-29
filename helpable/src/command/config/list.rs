use crate::config::Config;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct List {}

impl List {
    pub fn execute(&self, mut config: Config) -> Result<(), String> {
        for (key, value) in config.list() {
            println!("{}:{}", key, value);
        }
        Ok(())
    }
}
