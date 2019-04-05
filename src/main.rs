mod awsudo;

extern crate chrono;
extern crate clap;
extern crate dirs;
extern crate ini;
extern crate rusoto_core;
extern crate rusoto_sts;

use awsudo::credentials::fetcher::strategies::cache::Cache;
use awsudo::credentials::fetcher::strategies::request::Request;
use awsudo::credentials::fetcher::Fetcher;

use std::io;
use std::process::{Command, Stdio};

use awsudo::profile::Profile;

pub fn token_collector(mfa_serial: String) -> Option<String> {
    let mut buffer = String::new();
    println!("Please type your MFA token for {}: ", mfa_serial);
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read your input");

    let token: Option<String> = match buffer.trim().parse() {
        Ok(t) => Some(t),
        Err(_) => None,
    };

    token
}

fn main() {
    // Parse command arguments
    let args = awsudo::cli::parse();


    // Get Credentials to be injected
    // First, try to get credentials from Cache
    let cache = Cache {
        dir: args.cache_dir,
        profile: args.user.clone(),
    };
    let credentials = match cache.fetch() {
        Ok(credentials) => credentials,
        Err(_) => {
            // If that doesn't work, it tries then to request a new on from STS
            let profile = match Profile::load_from(args.config, args.user) {
                Ok(p) => p,
                Err(e) => panic!(e),
            };
            let r = Request {
                profile: profile,
                token_collector: token_collector,
            };

            match r.fetch() {
                Ok(credentials) => credentials,
                Err(e) => panic!(e),
            }
        }
    };

    //TODO Cache Token

    // Inject Environment Variables from Credentials
    credentials.inject();

    // Run the command with the Environment Credentials
    //TODO Extract this to its own module/file/package...
    Command::new("sh")
        .arg("-c")
        .arg(args.command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Something went wrong");
}
