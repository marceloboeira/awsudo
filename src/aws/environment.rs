use std::env;

pub struct Credentials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: String,
}

impl Credentials {
    pub fn inject(&self) {
        env::set_var("AWS_ACCESS_KEY_ID", &self.access_key_id);
        env::set_var("AWS_SECRET_ACCESS_KEY", &self.secret_access_key);
        env::set_var("AWS_SESSION_TOKEN", &self.session_token);
    }
}

pub fn inject(key: &str, secret: &str, token: &str) {
    Credentials {
        access_key_id: String::from(key),
        secret_access_key: String::from(secret),
        session_token: String::from(token),
    }
    .inject()
}

#[cfg(test)]
mod tests {
    use aws::environment::inject;
    use std::env;

    #[test]
    fn it_injects_credential_variable_to_env() {
        inject("m", "b", "j");

        assert_eq!(env::var("AWS_ACCESS_KEY_ID"), Ok("m".to_string()));
        assert_eq!(env::var("AWS_SECRET_ACCESS_KEY"), Ok("b".to_string()));
        assert_eq!(env::var("AWS_SESSION_TOKEN"), Ok("j".to_string()));
    }
}
