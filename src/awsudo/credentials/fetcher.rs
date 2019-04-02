pub mod strategies;

use awsudo::credentials::Credentials;

#[derive(Debug, PartialEq)]
pub enum Result {
    Error(String),
    Success(Credentials),
}

pub trait Fetcher {
    fn fetch(&self) -> Result;
}
