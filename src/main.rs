mod awsudo;

extern crate chrono;
extern crate clap;
extern crate dirs;
extern crate ini;
extern crate rusoto_core;
extern crate rusoto_sts;

use awsudo::credentials::fetcher::strategies::cache::Cache;
use awsudo::credentials::fetcher::Fetcher;
use rusoto_core::Region;
use rusoto_sts::{AssumeRoleRequest, Sts, StsClient};
use std::io;
use std::process::{Command, Stdio};

use awsudo::profile::Profile;

const AWS_DEFAULT_SESSION_NAME: &str = "awsudo";

fn main() {
    let args = awsudo::cli::parse();

    let cache = Cache {
        dir: args.cache_dir,
        profile: args.user.clone(),
    };

    match cache.fetch() {
        Ok(credentials) => credentials.inject(),
        _ => {
            let profile = match Profile::load_from(args.config, args.user) {
                Ok(p) => p,
                Err(e) => panic!(e),
            };

            let base_arr = AssumeRoleRequest {
                role_arn: profile.role_arn,
                role_session_name: AWS_DEFAULT_SESSION_NAME.to_owned(),
                ..Default::default()
            };

            let arr = match profile.mfa_serial {
                Some(serial) => {
                    let mut buffer = String::new();
                    if !serial.is_empty() {
                        println!("Please type your MFA token for {}: ", serial);

                        io::stdin()
                            .read_line(&mut buffer)
                            .expect("Failed to read your input");
                    }

                    let token: String = match buffer.trim().parse() {
                        Ok(t) => t,
                        Err(_) => panic!("error while reading the token"),
                    };

                    AssumeRoleRequest {
                        serial_number: Some(serial.to_string()),
                        token_code: Some(token.to_string()),
                        ..base_arr
                    }
                }
                None => base_arr,
            };

            // Token Generator
            //TODO Extract this to its own module/file/package...
            //TODO use the default
            //TODO handle token failures
            let sts = StsClient::new(Region::EuCentral1);
            match sts.assume_role(arr).sync() {
                Err(e) => panic!("{:?}", e),
                Ok(response) => {
                    let credentials = response.credentials.unwrap();
                    awsudo::environment::inject(
                        credentials.access_key_id.as_str(),
                        credentials.secret_access_key.as_str(),
                        credentials.session_token.as_str(),
                    )
                }
            };
        }
    }

    // Command runner
    //TODO Extract this to its own module/file/package...
    Command::new("sh")
        .arg("-c")
        .arg(args.command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Something went wrong");
}
