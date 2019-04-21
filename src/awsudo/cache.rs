extern crate chrono;
extern crate ini;

use self::chrono::{DateTime, Duration, Utc};
use self::ini::Ini;

use awsudo::credentials::Credentials;
use awsudo::fetcher::Fetcher;
use std::fs;
use std::path::PathBuf;

pub struct Cache {
    pub dir: PathBuf,
    pub file: String,
}

impl Cache {
    pub fn new(dir: PathBuf, filename: &str) -> Cache {
        Cache { dir, file: filename.to_owned() }
    }
}

impl Cache {
    pub fn persist(&self, credentials: Credentials) -> Result<(), &'static str> {
        if credentials.cached {
            Ok(())
        } else {
            match fs::create_dir_all(&self.dir) {
                Ok(_) => {
                    let path = self.dir.join(&self.file);
                    //TODO move this logic to credentials (read from STS request)
                    let expires_at = (Utc::now() + Duration::hours(1)).to_rfc3339();

                    let mut conf = Ini::new();
                    conf.with_section(None::<String>)
                        .set("ACCESS_KEY_ID", credentials.access_key_id)
                        .set("SECRET_ACCESS_KEY", credentials.secret_access_key)
                        .set("SESSION_TOKEN", credentials.session_token)
                        .set("SESSION_EXPIRES_AT", expires_at);

                    match conf.write_to_file(path) {
                        Ok(_) => Ok(()),
                        Err(_) => Err("Failed to persist cache: file cannot be created"),
                    }
                }
                Err(_) => Err("Failed to persist cache: dir cannot be created"),
            }
        }
    }
}

impl Fetcher for Cache {
    fn fetch(&self) -> Result<Credentials, &'static str> {
        match Ini::load_from_file(self.dir.join(&self.file)) {
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
    use std::fs;
    use std::path::{Path, PathBuf};

    fn fixtures_tmp_path() -> String {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("test/fixtures/tmp/cache");
        p.to_str().unwrap().to_string()
    }

    fn fixtures_path() -> String {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("test/fixtures/cache");
        p.to_str().unwrap().to_string()
    }

    #[test]
    fn it_returns_error_when_the_file_is_not_present() {
        assert_eq!(
            Cache::new(fixtures_path(), "path".to_string()).fetch(),
            Err("Cache file is not present or not valid")
        );
    }

    #[test]
    fn it_returns_error_when_the_file_is_not_ini_valid() {
        assert_eq!(
            Cache::new(fixtures_path(), "invalid".to_string()).fetch(),
            Err("Cache file is missing required values")
        );
    }

    #[test]
    fn it_returns_error_when_the_file_is_missing_values_valid() {
        assert_eq!(
            Cache::new(fixtures_path(), "invalid_missing_values".to_string()).fetch(),
            Err("Cache file is missing required values")
        );
    }

    #[test]
    fn it_returns_error_when_the_file_date_is_not_valid() {
        assert_eq!(
            Cache::new(fixtures_path(), "invalid_date".to_string()).fetch(),
            Err("Cache file does not have a valid date")
        );
    }

    #[test]
    fn it_returns_error_when_the_file_date_is_expired() {
        assert_eq!(
            Cache::new(fixtures_path(), "invalid_expired".to_string()).fetch(),
            Err("Cache file is expired")
        );
    }

    #[test]
    fn it_returns_the_credentails_when_the_valid() {
        assert_eq!(
            Cache::new(fixtures_path(), "valid".to_string()).fetch(),
            Ok(Credentials {
                access_key_id: "ASIA3NOTVALID2WN5".to_string(),
                secret_access_key: "8s7k+21mKladUU9d".to_string(),
                session_token: "AgoGb3JpZ2luECwaDGV1LW".to_string(),
                cached: true,
            }),
        );
    }

    #[test]
    fn it_returns_ok_when_the_credentials_are_already_cached() {
        let cr = Credentials {
            access_key_id: "-".to_string(),
            secret_access_key: "-".to_string(),
            session_token: "-".to_string(),
            cached: true,
        };

        assert_eq!(
            Cache::new(fixtures_tmp_path(), "it-doesnt-matter".to_string()).persist(cr),
            Ok(()),
        );
    }

    #[test]
    fn it_returns_err_when_the_path_does_not_exist_and_cannot_be_created() {
        let cr = Credentials {
            access_key_id: "-".to_string(),
            secret_access_key: "-".to_string(),
            session_token: "-".to_string(),
            cached: false,
        };

        assert_eq!(
            Cache::new("/invalid".to_string(), "it-doesnt-matter".to_string()).persist(cr),
            Err("Failed to persist cache: dir cannot be created"),
        );
    }

    #[test]
    fn it_creates_the_file_when_everything_is_correct() {
        let cr = Credentials {
            access_key_id: "A23".to_string(),
            secret_access_key: "M07".to_string(),
            session_token: "B03".to_string(),
            cached: false,
        };

        assert_eq!(
            Cache::new(fixtures_tmp_path(), "file".to_string()).persist(cr),
            Ok(()),
        );

        assert_eq!(Path::new(&fixtures_tmp_path()).join("file").exists(), true);

        fs::remove_dir_all(fixtures_tmp_path()).unwrap();
    }
}
