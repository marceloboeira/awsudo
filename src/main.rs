extern crate clap;
extern crate ini;
extern crate rusoto_core;
extern crate rusoto_sts;

use std::env;
use std::path::Path;
use std::io;
use std::process::{Command, Stdio};

use ini::Ini;
use clap::{Arg, App};
use rusoto_sts::{Sts, StsClient, Credentials, AssumeRoleRequest};
use rusoto_core::{Region};

const AWS_DEFAULT_CONFIG_PATH: &str = ".aws/credentials";
const AWS_DEFAULT_SESSION_NAME: &str = "awsudo";

fn inject_environment_with(credentials: Credentials) {
    env::set_var("AWS_ACCESS_KEY_ID", credentials.access_key_id);
    env::set_var("AWS_SECRET_ACCESS_KEY", credentials.secret_access_key);
    env::set_var("AWS_SESSION_TOKEN", credentials.session_token);
}

fn main() {
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

    println!("config-file: {:?}", config_file_path);
    println!("profile: {:?}", profile_name);
    println!("command: {:?}", command);

    let conf = match Ini::load_from_file(config_file_path) {
        Err(message) => panic!(message),
        Ok(value) => value
    };

    let section = conf.section(Some(profile_name)).unwrap();
    let role_arn = section.get("role_arn").unwrap();
    let mfa_serial = section.get("mfa_serial").unwrap();
    // parse region or default

    println!("role_arn: {:?}", role_arn);
    println!("mfa_serial: {:?}", mfa_serial);

    // Get the MFA token if necessary
    println!("Please type your MFA token for {:}: ", mfa_serial);
    let mut token = String::new();
    match io::stdin().read_line(&mut token) {
        Ok(_) => {
            token.pop(); //REMOVES THE \n
            println!("token: {}", token);
        }
        Err(error) => println!("error: {}", error),
    }

    let sts = StsClient::new(Region::EuCentral1);

    //load thiings from Profile
    match sts.assume_role(AssumeRoleRequest{
            role_arn: role_arn.to_string(),
            role_session_name: AWS_DEFAULT_SESSION_NAME.to_owned(),
            serial_number: Some(mfa_serial.to_string()),
            token_code: Some(token.to_string()),
            ..Default::default() }).sync() {
        Err(e) => panic!("{:?}", e),
        Ok(response) => inject_environment_with(response.credentials.unwrap())
    };

    Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Something went wrong");
}
