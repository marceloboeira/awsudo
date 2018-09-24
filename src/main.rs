extern crate clap;
extern crate ini;
extern crate rusoto_core;
extern crate rusoto_sts;
extern crate chrono;

use std::env;
use std::path::Path;
use std::io;
use std::process::{Command, Stdio};

use ini::Ini;
use clap::{Arg, App};
use rusoto_sts::{Sts, StsClient, AssumeRoleRequest};
use rusoto_core::{Region};
use chrono::prelude::*;

const AWS_DEFAULT_CONFIG_PATH: &str = ".aws/credentials";
const AWS_DEFAULT_SESSION_NAME: &str = "awsudo";

fn main() {
    // CLI
    //TODO Extract this to its own module/file/package...
    let matches = App::new("awsudo - sudo-like behavior for role assumed access on AWS accounts")
                          .version("0.1")
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Sets a custom config file other than ~/.aws/credentials")
                               .takes_value(true))
                          .arg(Arg::with_name("user")
                               .short("u")
                               .long("user")
                               .help("Set the AWS profile name based on the config file")
                               .required(true)
                               .takes_value(true))
                          .arg(Arg::with_name("command")
                               .help("The command to run with the assumed role")
                               .required(true)
                               .takes_value(true)).get_matches();

    let mut default_config_file_path = match env::home_dir() {
        Some(path) => path,
        None => panic!("Something wrong with your home dir")
    };
    default_config_file_path.push(AWS_DEFAULT_CONFIG_PATH);

    let config_file_path = match matches.value_of("config") {
        Some(value) => Path::new(value),
        None => default_config_file_path.as_path()
    };

    let profile_name = matches.value_of("user").unwrap_or("default");
    let command = matches.value_of("command").unwrap_or("--");

    // END CLI

    //  State manager
    let conf = match Ini::load_from_file(config_file_path.clone()) {
        Err(message) => panic!(message),
        Ok(value) => value
    };

    let section = conf.section(Some(profile_name)).unwrap();
    let role_arn = section.get("role_arn").unwrap();
    let mfa_serial = section.get("mfa_serial").unwrap();
    //TODO parse region or default

    // ~~~cached token ~~
    let file_aws_access_key_id = match section.get("aws_sudo_access_key_id") {
        Some(value) => value,
        None => "invalid"
    };
    let file_aws_secret_access_key = match section.get("aws_sudo_secret_access_key") {
        Some(value) => value,
        None => "invalid"
    };
    let file_aws_session_token = match section.get("aws_sudo_session_token") {
        Some(value) => value,
        None => "invalid"
    };
    let file_aws_session_expiration_date = match section.get("aws_sudo_session_expiration_date") {
        Some(value) => value,
        None => "1994-07-03T07:30:00.00Z"
    };

    let now = Utc::now();
    let session_expires_at = match file_aws_session_expiration_date.parse::<DateTime<Utc>>() {
        Ok(value) => value,
        Err(e) => panic!("{:?}", e)
    };

    println!("role_arn: {:?}", role_arn);
    println!("mfa_serial: {:?}", mfa_serial);

    if file_aws_access_key_id != "invalid" && now <= session_expires_at {
        env::set_var("AWS_ACCESS_KEY_ID", file_aws_access_key_id);
        env::set_var("AWS_SECRET_ACCESS_KEY", file_aws_secret_access_key);
        env::set_var("AWS_SESSION_TOKEN", file_aws_session_token);
    } else {
        //TODO use a debug flag for those
        println!("role_arn: {:?}", role_arn);
        println!("mfa_serial: {:?}", mfa_serial);
        // END State Manager

        //TODO Figure where to put this token request interaction...
        //TODO Get the MFA token only if necessary
        let mut token = String::new();
        if !mfa_serial.is_empty() {
            println!("Please type your MFA token for {:}: ", mfa_serial);
            match io::stdin().read_line(&mut token) {
                Ok(_) => {
                    token.pop(); //REMOVES THE \n
                    println!("token: {}", token);
                }
                Err(error) => println!("error: {}", error),
            }
        }

        // Token Generator
        //TODO Extract this to its own module/file/package...
        //TODO use the default
        let sts = StsClient::new(Region::EuCentral1);
        match sts.assume_role(AssumeRoleRequest{
            role_arn: role_arn.to_string(),
            role_session_name: AWS_DEFAULT_SESSION_NAME.to_owned(),
            serial_number: Some(mfa_serial.to_string()),
            token_code: Some(token.to_string()),
            ..Default::default() }).sync() {
            Err(e) => panic!("{:?}", e),
            Ok(response) => {
                let credentials = response.credentials.unwrap();
                let mut another = match Ini::load_from_file(config_file_path.clone()) {
                    Err(message) => panic!(message),
                    Ok(value) => value
                };

                another.with_section(Some(profile_name))
                    .set("aws_sudo_access_key_id", credentials.access_key_id.clone())
                    .set("aws_sudo_secret_access_key", credentials.secret_access_key.clone())
                    .set("aws_sudo_session_token", credentials.session_token.clone())
                    .set("aws_sudo_session_expiration_date", credentials.expiration.clone());

                another.write_to_file(config_file_path.clone()).unwrap();

                env::set_var("AWS_ACCESS_KEY_ID", credentials.access_key_id);
                env::set_var("AWS_SECRET_ACCESS_KEY", credentials.secret_access_key);
                env::set_var("AWS_SESSION_TOKEN", credentials.session_token);
            }

        };
    }

    // Command runner
    //TODO Extract this to its own module/file/package...
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Something went wrong");
}
