use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct List {}

impl List {
    pub fn execute(&self) -> Result<(), String> {
        Ok(())
    }
}
