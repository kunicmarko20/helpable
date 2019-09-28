use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Set {
    /// Name of the config that is modified
    pub name: String,
    /// New value of the config
    pub value: String,
}

impl Set {
    pub fn execute(&self) -> Result<(), String> {
        println!("{}:{}", self.name, self.value);
        Ok(())
    }
}
