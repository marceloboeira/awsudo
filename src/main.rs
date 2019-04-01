mod awsudo;

extern crate chrono;
extern crate clap;
extern crate dirs;
extern crate ini;
extern crate rusoto_core;
extern crate rusoto_sts;

use std::io;
use std::process::{Command, Stdio};

use chrono::prelude::*;
use ini::Ini;
use rusoto_core::Region;
use rusoto_sts::{AssumeRoleRequest, Sts, StsClient};

const AWS_DEFAULT_SESSION_NAME: &str = "awsudo";

fn main() {
    let args = awsudo::cli::parse();

    //  State manager
    let conf = match Ini::load_from_file(args.config.clone()) {
        Err(message) => panic!(message),
        Ok(value) => value,
    };

    let section = conf.section(Some(args.user.clone())).unwrap();
    let role_arn = section.get("role_arn").unwrap();
    let mfa_serial = section.get("mfa_serial");
    //TODO parse region or default

    // ~~~cached token ~~
    let file_aws_access_key_id = match section.get("aws_sudo_access_key_id") {
        Some(value) => value,
        None => "invalid",
    };
    let file_aws_secret_access_key = match section.get("aws_sudo_secret_access_key") {
        Some(value) => value,
        None => "invalid",
    };
    let file_aws_session_token = match section.get("aws_sudo_session_token") {
        Some(value) => value,
        None => "invalid",
    };
    let file_aws_session_expiration_date = match section.get("aws_sudo_session_expiration_date") {
        Some(value) => value,
        None => "1994-07-03T07:30:00.00Z",
    };

    let now = Utc::now();
    let session_expires_at = match file_aws_session_expiration_date.parse::<DateTime<Utc>>() {
        Ok(value) => value,
        Err(e) => panic!("{:?}", e),
    };

    if file_aws_access_key_id != "invalid" && now <= session_expires_at {
        awsudo::environment::inject(
            file_aws_access_key_id,
            file_aws_secret_access_key,
            file_aws_session_token,
        )
    } else {
        let base_arr = AssumeRoleRequest {
            role_arn: role_arn.to_string(),
            role_session_name: AWS_DEFAULT_SESSION_NAME.to_owned(),
            ..Default::default()
        };

        let arr = match mfa_serial {
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
                let mut another = match Ini::load_from_file(args.config.clone()) {
                    Err(message) => panic!(message),
                    Ok(value) => value,
                };

                another
                    .with_section(Some(args.user.clone()))
                    .set("aws_sudo_access_key_id", credentials.access_key_id.clone())
                    .set(
                        "aws_sudo_secret_access_key",
                        credentials.secret_access_key.clone(),
                    )
                    .set("aws_sudo_session_token", credentials.session_token.clone())
                    .set(
                        "aws_sudo_session_expiration_date",
                        credentials.expiration.clone(),
                    );

                another.write_to_file(args.config.clone()).unwrap();

                awsudo::environment::inject(
                    credentials.access_key_id.as_str(),
                    credentials.secret_access_key.as_str(),
                    credentials.session_token.as_str(),
                )
            }
        };
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
