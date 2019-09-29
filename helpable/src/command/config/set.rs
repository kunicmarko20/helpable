use crate::config::Config;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Set {
    /// Name of the config that is modified
    pub key: String,
    /// New value of the config
    pub value: String,
}

impl Set {
    pub fn execute(&mut self, mut config: Config) -> Result<(), String> {
        config.set(&self.key, self.value.clone());
        config.save();
        Ok(())
    }
}
