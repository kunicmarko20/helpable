use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Release {}

impl Release {
    pub fn execute(&self) {

    }
}
