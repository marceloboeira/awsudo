extern crate ini;

use awsudo::credentials::fetcher::{Fetcher, Result};
use awsudo::credentials::Credentials;
use chrono::{DateTime, Utc};
use ini::Ini;
use std::path::Path;

pub struct Cache {
    pub dir: String,
    pub profile: String,
}

impl Fetcher for Cache {
    fn fetch(&self) -> Result {
        match Ini::load_from_file(Path::new(&self.dir).join(&self.profile)) {
            Err(_) => Result::Error("Cache file is not present or not valid".to_string()),
            Ok(ini_file) => {
                let section = ini_file.general_section();

                match (
                    section.get(&"ACCESS_KEY_ID".to_string()),
                    section.get(&"SECRET_ACCESS_KEY".to_string()),
                    section.get(&"SESSION_TOKEN".to_string()),
                    section.get(&"SESSION_EXPIRES_AT".to_string()),
                ) {
                    (
                        Some(access_key_id),
                        Some(secret_access_key),
                        Some(session_token),
                        Some(session_expires_at_raw),
                    ) => match session_expires_at_raw.parse::<DateTime<Utc>>() {
                        Ok(session_expires_at) => {
                            if session_expires_at > Utc::now() {
                                Result::Success(Credentials {
                                    access_key_id: access_key_id.clone(),
                                    secret_access_key: secret_access_key.clone(),
                                    session_token: session_token.clone(),
                                })
                            } else {
                                Result::Error("Cache file is expired".to_string())
                            }
                        }
                        Err(_) => {
                            Result::Error("Cache file does not have a valid date".to_string())
                        }
                    },
                    (_, _, _, _) => {
                        Result::Error("Cache file is missing required values".to_string())
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use awsudo::credentials::fetcher::strategies::cache;
    use awsudo::credentials::fetcher::{Fetcher, Result};
    use awsudo::credentials::Credentials;
    use std::path::PathBuf;

    fn fixtures_path() -> String {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("test/fixtures");
        p.to_str().unwrap().to_string()
    }

    #[test]
    fn it_returns_error_when_the_file_is_not_present() {
        let c = cache::Cache {
            dir: "invalid".to_string(),
            profile: "path".to_string(),
        };

        assert_eq!(
            c.fetch(),
            Result::Error("Cache file is not present or not valid".to_string()),
        );
    }

    #[test]
    fn it_returns_error_when_the_file_is_not_ini_valid() {
        let c = cache::Cache {
            dir: fixtures_path(),
            profile: "invalid".to_string(),
        };

        assert_eq!(
            c.fetch(),
            Result::Error("Cache file is missing required values".to_string()),
        );
    }

    #[test]
    fn it_returns_error_when_the_file_is_missing_values_valid() {
        let c = cache::Cache {
            dir: fixtures_path(),
            profile: "invalid_missing_values".to_string(),
        };

        assert_eq!(
            c.fetch(),
            Result::Error("Cache file is missing required values".to_string()),
        );
    }

    #[test]
    fn it_returns_error_when_the_file_date_is_not_valid() {
        let c = cache::Cache {
            dir: fixtures_path(),
            profile: "invalid_date".to_string(),
        };

        assert_eq!(
            c.fetch(),
            Result::Error("Cache file does not have a valid date".to_string()),
        );
    }

    #[test]
    fn it_returns_error_when_the_file_date_is_expired() {
        let c = cache::Cache {
            dir: fixtures_path(),
            profile: "invalid_expired".to_string(),
        };

        assert_eq!(
            c.fetch(),
            Result::Error("Cache file is expired".to_string()),
        );
    }

    #[test]
    fn it_returns_the_credentails_when_the_valid() {
        let c = cache::Cache {
            dir: fixtures_path(),
            profile: "valid".to_string(),
        };

        assert_eq!(
            c.fetch(),
            Result::Success(Credentials {
                access_key_id: "ASIA3NOTVALID2WN5".to_string(),
                secret_access_key: "8s7k+21mKladUU9d".to_string(),
                session_token: "AgoGb3JpZ2luECwaDGV1LW".to_string(),
            }),
        );
    }
}
