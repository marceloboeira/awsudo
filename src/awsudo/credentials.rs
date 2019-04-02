pub mod fetcher;

#[derive(Debug, PartialEq)]
pub struct Credentials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: String,
}
