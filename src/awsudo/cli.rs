extern crate clap;
extern crate dirs;

use clap::{App, AppSettings, Arg, ArgMatches};

const AWS_DEFAULT_CONFIG_PATH: &str = ".aws/config";

pub struct CLI {
    pub user: String,
    pub command: String,
    pub config: String,
}

pub fn parse() -> CLI {
    from_args(default().get_matches())
}

fn from_args(matches: ArgMatches) -> CLI {
    let user = String::from(matches.value_of("user").unwrap_or("default"));
    let config: String = match matches.value_of("config") {
        Some(value) => String::from(value),
        None => match dirs::home_dir() {
            Some(path) => match path.join(AWS_DEFAULT_CONFIG_PATH).to_str() {
                Some(s) => String::from(s),
                None => panic!("Something wrong with your home dir"),
            },
            None => panic!("Something wrong with your home dir"),
        },
    };
    let command = match matches.subcommand() {
        (external, maybe_matches) => {
            let args = match maybe_matches {
                Some(external_matches) => match external_matches.values_of("") {
                    Some(values) => values.collect::<Vec<&str>>().join(" "),
                    None => String::from(""),
                },
                _ => String::from(""),
            };

            String::from(vec![String::from(external), args].join(" ").trim())
        }
    };

    CLI {
        user,
        config,
        command,
    }
}

fn default<'b, 'c>() -> App<'b, 'c> {
    App::new("awsudo - sudo-like behavior for role assumed access on AWS accounts")
        .version("0.1")
        .setting(AppSettings::AllowExternalSubcommands)
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file other than ~/.aws/credentials")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("user")
                .short("u")
                .long("user")
                .help("Set the AWS profile name based on the config file")
                .required(true)
                .takes_value(true),
        )
}

#[cfg(test)]
mod tests {
    use awsudo::cli;

    #[test]
    fn it_parses_user() {
        let result = cli::from_args(cli::default().get_matches_from(vec!["awsudo", "-u", "jeff"]));

        assert_eq!(result.user, "jeff");
    }

    #[test]
    fn it_sets_default_config() {
        let result = cli::from_args(cli::default().get_matches_from(vec!["awsudo", "-u", "jeff"]));

        assert_eq!(
            result.config,
            dirs::home_dir()
                .unwrap()
                .join(".aws/config")
                .to_str()
                .unwrap()
        );
    }

    #[test]
    fn it_parses_config() {
        let result = cli::from_args(cli::default().get_matches_from(vec![
            "awsudo",
            "-u",
            "jeff",
            "-c",
            "/usr/specific/path",
        ]));

        assert_eq!(result.config, "/usr/specific/path");
    }

    #[test]
    fn it_parses_single_command() {
        let result =
            cli::from_args(cli::default().get_matches_from(vec!["awsudo", "-u", "jeff", "echo"]));

        assert_eq!(result.command, "echo");
    }

    #[test]
    fn it_parses_command_with_multiple_words() {
        let result = cli::from_args(
            cli::default().get_matches_from(vec!["awsudo", "-u", "jeff", "echo", "bezos", "aws"]),
        );

        assert_eq!(result.command, "echo bezos aws");
    }

    #[test]
    fn it_parses_command_with_attribute() {
        let result =
            cli::from_args(cli::default().get_matches_from(vec!["awsudo", "-u", "jeff", "ls",  "-a"]));

        assert_eq!(result.command, "ls -a");
    }

    #[test]
    fn it_parses_command_with_multiple_attributes() {
        let result = cli::from_args(
            cli::default().get_matches_from(vec!["awsudo", "-u", "jeff", "ls", "-a", "-l"]),
        );

        assert_eq!(result.command, "ls -a -l");
    }
}