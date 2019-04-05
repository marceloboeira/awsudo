use awsudo::credentials::Credentials;

pub trait Fetcher {
    fn fetch(&self) -> Result<Credentials, &'static str>;
}
