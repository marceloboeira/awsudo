mod awsudo;

use awsudo::cache::Cache;
use awsudo::cli;
use awsudo::dispatcher;
use awsudo::fetcher::Fetcher;
use awsudo::profile::Profile;
use awsudo::request::Request;

use std::io;

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
    let args = cli::parse();

    // Get Credentials to be injected
    // First, try to get credentials from Cache
    let cache = Cache::new(args.cache_dir, &args.user);
    let credentials = match cache.fetch() {
        Ok(credentials) => credentials,
        Err(_) => {
            // If that doesn't work, it tries then to request a new on from STS
            match Profile::load_from(args.config, args.user) {
                Ok(p) => match Request::new(p, token_collector).fetch() {
                    Ok(credentials) => credentials,
                    Err(e) => panic!(e),
                },
                Err(e) => panic!(e),
            }
        }
    };

    // Inject Environment Variables from Credentials
    credentials.inject();

    // Persist Credentials on Cache
    match cache.persist(credentials) {
        Err(_) => println!("Failed to cache"),
        Ok(_) => (),
    };

    // Run the command with the Environment Credentials
    dispatcher::run(args.command);
}
