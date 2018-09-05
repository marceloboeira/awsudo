# > awsudo
> sudo-like behavior for role assumed access on AWS accounts

A rewrite of the core behavior of [makethunder/awsudo](https://github.com/makethunder/awsudo/) in [rust](https://github.com/rust-lang/rust).

:warning: **IMPORTANT** the current version is just an experiment, I plan to split the code and test it well.

See the [motivation](#motivation) for more info.

## Usage

```
awsudo - sudo-like behavior for AWS accounts

USAGE:
    awsudo [OPTIONS] <command> --user <user>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>    Sets a custom config file other than ~/.aws/credentials
    -u, --user <user>      Set the AWS profile name based on the config file

ARGS:
    <command>    The command to run with the assumed role
```

Thanks [clap](https://github.com/clap-rs/clap) for that.

For example, to get all of the S3 buckets of the **production** account:
```
awsudo -u production 'aws s3 ls'
```

For now, we have some limitations:
1. The command needs to be quoted
1. A new session/token is created everytime
1. Only works with MFA (afaik)

Probably tackling them soon, since it doesn't make sense for me to replace the current one without those

## Motivation

Main motivation was to write something that I would use everyday with rust.

The [original awsudo](https://github.com/makethunder/awsudo/) is heavily used where I work and it constantly causes pain, the CLI has a couple of issues:
1. **Distribution** - It was written in Python, which makes it difficult to distribute, also doens't have a homebrew formula
1. **Dependencies** - It [locks the aws-cli version](https://github.com/makethunder/awsudo/issues/7), and [depends on code of the CLI itsel](https://github.com/makethunder/awsudo/blob/d5800bc4a9785d179c678605d0ae5bf4e28f5205/awsudo/config.py#L1)
1. **Versioning** - It [doesn't have versions whatsoever](https://github.com/makethunder/awsudo/releases)
1. **Bugs** - It has a couple of bugs, e.g.: [you can't pass `AWS_*` like variables to your command](https://github.com/makethunder/awsudo/issues/14)

The [official solution](https://docs.aws.amazon.com/cli/latest/userguide/cli-roles.html#cli-roles-cache) from AWS is not really great either:
1. Stateful - Once you assume you stick with it until it expires or you switch
1. CLI centric - Hard to share the credentials and switch quickly if you are not using their CLI
