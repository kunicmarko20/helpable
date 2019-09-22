#[macro_use]
extern crate structopt;

use crate::helpable::Helpable;
use crate::helpable::HelpableSubCommand;
use crate::structopt::StructOpt;

mod command;
mod helpable;

fn main() {
    match Helpable::from_args().command {
        HelpableSubCommand::Release(command) => command.execute(),
    }
}
