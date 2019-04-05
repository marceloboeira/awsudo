use awsudo::credentials::Credentials;
use std::env;

impl Credentials {
    pub fn inject(&self) {
        env::set_var("AWS_ACCESS_KEY_ID", &self.access_key_id);
        env::set_var("AWS_SECRET_ACCESS_KEY", &self.secret_access_key);
        env::set_var("AWS_SESSION_TOKEN", &self.session_token);
    }
}

#[cfg(test)]
mod tests {
    use awsudo::credentials::Credentials;
    use std::env;

    #[test]
    fn it_injects_credential_variable_to_env() {
        Credentials {
            access_key_id: "m".to_string(),
            secret_access_key: "b".to_string(),
            session_token: "j".to_string(),
            cached: false,
        }
        .inject();

        assert_eq!(env::var("AWS_ACCESS_KEY_ID"), Ok("m".to_string()));
        assert_eq!(env::var("AWS_SECRET_ACCESS_KEY"), Ok("b".to_string()));
        assert_eq!(env::var("AWS_SESSION_TOKEN"), Ok("j".to_string()));
    }
}
