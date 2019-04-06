extern crate ini;

use awsudo::credentials::Credentials;
use awsudo::fetcher::Fetcher;
use chrono::{DateTime, Utc};
use ini::Ini;
use std::path::Path;

pub struct Cache {
    pub dir: String,
    pub file: String,
}

impl Cache {
    pub fn new(dir: String, file: String) -> Cache {
        Cache { dir, file }
    }
}

impl Fetcher for Cache {
    fn fetch(&self) -> Result<Credentials, &'static str> {
        match Ini::load_from_file(Path::new(&self.dir).join(&self.file)) {
            Err(_) => Err("Cache file is not present or not valid"),
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
                                Ok(Credentials {
                                    access_key_id: access_key_id.clone(),
                                    secret_access_key: secret_access_key.clone(),
                                    session_token: session_token.clone(),
                                    cached: true,
                                })
                            } else {
                                Err("Cache file is expired")
                            }
                        }
                        Err(_) => Err("Cache file does not have a valid date"),
                    },
                    (_, _, _, _) => Err("Cache file is missing required values"),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use awsudo::cache::Cache;
    use awsudo::credentials::Credentials;
    use awsudo::fetcher::Fetcher;
    use std::path::PathBuf;

    fn fixtures_path() -> String {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("test/fixtures/cache");
        p.to_str().unwrap().to_string()
    }

    #[test]
    fn it_returns_error_when_the_file_is_not_present() {
        let c = Cache {
            dir: "invalid".to_string(),
            file: "path".to_string(),
        };

        assert_eq!(c.fetch(), Err("Cache file is not present or not valid"));
    }

    #[test]
    fn it_returns_error_when_the_file_is_not_ini_valid() {
        let c = Cache {
            dir: fixtures_path(),
            file: "invalid".to_string(),
        };

        assert_eq!(c.fetch(), Err("Cache file is missing required values"));
    }

    #[test]
    fn it_returns_error_when_the_file_is_missing_values_valid() {
        let c = Cache {
            dir: fixtures_path(),
            file: "invalid_missing_values".to_string(),
        };

        assert_eq!(c.fetch(), Err("Cache file is missing required values"));
    }

    #[test]
    fn it_returns_error_when_the_file_date_is_not_valid() {
        let c = Cache {
            dir: fixtures_path(),
            file: "invalid_date".to_string(),
        };

        assert_eq!(c.fetch(), Err("Cache file does not have a valid date"));
    }

    #[test]
    fn it_returns_error_when_the_file_date_is_expired() {
        let c = Cache {
            dir: fixtures_path(),
            file: "invalid_expired".to_string(),
        };

        assert_eq!(c.fetch(), Err("Cache file is expired"));
    }

    #[test]
    fn it_returns_the_credentails_when_the_valid() {
        let c = Cache {
            dir: fixtures_path(),
            file: "valid".to_string(),
        };

        assert_eq!(
            c.fetch(),
            Ok(Credentials {
                access_key_id: "ASIA3NOTVALID2WN5".to_string(),
                secret_access_key: "8s7k+21mKladUU9d".to_string(),
                session_token: "AgoGb3JpZ2luECwaDGV1LW".to_string(),
                cached: true,
            }),
        );
    }
}
