#[macro_use]
extern crate serde_derive;

mod client;
pub mod payload;

pub use self::client::{JenkinsClient, JenkinsConfig};
