use rusoto_core::Region;
use rusoto_sts::{AssumeRoleRequest, Sts, StsClient};

use awsudo::credentials::fetcher::Fetcher;
use awsudo::credentials::Credentials;
use awsudo::profile::Profile;

const AWS_DEFAULT_SESSION_NAME: &str = "awsudo";

pub struct Request {
    pub profile: Profile,
    pub token_collector: fn(String) -> Option<String>,
}

impl Fetcher for Request {
    fn fetch(&self) -> Result<Credentials, &'static str> {
        let base_request = AssumeRoleRequest {
            role_arn: self.profile.role_arn.clone(),
            role_session_name: AWS_DEFAULT_SESSION_NAME.to_owned(),
            ..Default::default()
        };

        let request = match self.profile.mfa_serial.clone() {
            Some(serial) => match (self.token_collector)(serial.clone()) {
                Some(token) => AssumeRoleRequest {
                    serial_number: Some(serial.to_string()),
                    token_code: Some(token.to_string()),
                    ..base_request
                },
                None => base_request,
            },
            None => base_request,
        };

        match StsClient::new(Region::EuCentral1)
            .assume_role(request)
            .sync()
        {
            Err(_) => Err("Request to AWS failed"),
            Ok(response) => match response.credentials {
                Some(c) => Ok(Credentials {
                    access_key_id: c.access_key_id,
                    secret_access_key: c.secret_access_key,
                    session_token: c.session_token,
                }),
                None => Err("Request to AWS failed"),
            },
        }
    }
}

//TODO: Find a way to properly test this
// Right now it is a bit tricky considering the external types/requests/side-effects
