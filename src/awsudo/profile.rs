extern crate ini;

use ini::Ini;
use std::path::Path;

#[derive(Debug)]
pub struct Profile {
    pub role_arn: String,
    pub region: String,
    pub mfa_serial: Option<String>,
}

impl PartialEq for Profile {
    fn eq(&self, other: &Profile) -> bool {
        self.role_arn == other.role_arn
            && self.region == other.region
            && self.mfa_serial == other.mfa_serial
    }
}

impl Profile {
    pub fn load_from(file_path: String, user: String) -> Result<Profile, &'static str> {
        match Ini::load_from_file(Path::new(&file_path)) {
            Err(_) => Err("Profile file not found"),
            Ok(ini) => match ini.section(Some(user.to_owned())) {
                Some(s) => match (s.get("role_arn"), s.get("mfa_serial"), s.get("region")) {
                    (None, _, _) => Err("Profile role_arn not found"),
                    (Some(role_arn), mfa, region) => Ok(Profile {
                        role_arn: role_arn.to_string(),
                        mfa_serial: match mfa {
                            None => None,
                            Some(s) => Some(s.to_string()),
                        },
                        region: match region {
                            Some(r) => r.to_string(),
                            None => "eu-central-1".to_string(),
                        },
                    }),
                },
                None => Err("Profile not found"),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use awsudo::profile::Profile;
    use std::path::PathBuf;

    fn fixtures_path(file: &str) -> String {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("test/fixtures/config/");
        p.push(file);
        p.to_str().unwrap().to_string()
    }

    #[test]
    fn it_returns_an_error_when_file_is_not_found() {
        let r = Profile::load_from(fixtures_path("unexistent"), "staging".to_string());

        assert_eq!(r, Err("Profile file not found"));
    }

    #[test]
    fn it_returns_an_error_when_section_is_not_found() {
        let r = Profile::load_from(fixtures_path("multi_profile"), "staging".to_string());

        assert_eq!(r, Err("Profile not found"));
    }

    #[test]
    fn it_returns_an_error_when_arn_is_not_found() {
        let r = Profile::load_from(fixtures_path("missing_values"), "missing_arn".to_string());

        assert_eq!(r, Err("Profile role_arn not found"));
    }

    #[test]
    fn it_returns_none_when_mfa_is_not_found() {
        let r = Profile::load_from(fixtures_path("missing_values"), "missing_mfa".to_string());

        assert_eq!(
            r,
            Ok(Profile {
                mfa_serial: None,
                role_arn: String::from("example-arn"),
                region: String::from("us-east-1"),
            },)
        );
    }

    #[test]
    fn it_returns_default_when_region_is_not_found() {
        let r = Profile::load_from(
            fixtures_path("missing_values"),
            "missing_region".to_string(),
        );

        assert_eq!(
            r,
            Ok(Profile {
                mfa_serial: Some(String::from("example-mfa")),
                role_arn: String::from("example-arn"),
                region: String::from("eu-central-1"),
            },)
        );
    }

    #[test]
    fn it_returns_mfa_when_mfa_is_found() {
        let r = Profile::load_from(fixtures_path("multi_profile"), "complete".to_string());

        assert_eq!(
            r,
            Ok(Profile {
                mfa_serial: Some(String::from("example-mfa")),
                role_arn: String::from("example-arn"),
                region: String::from("us-east-1"),
            },)
        );
    }
}
