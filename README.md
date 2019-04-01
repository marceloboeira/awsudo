<p align="center">
  <img src="https://raw.githubusercontent.com/marceloboeira/awsudo/master/docs/logos/logo-transparent.png" width="300">
  <p align="center">sudo-like behavior for role assumed access on AWS accounts<p>
</p>

:warning: **IMPORTANT** the current version is just an experiment, I plan to split the code and test it well.

## Motivation

Main motivation was to write something that I would use everyday with rust.

The [original awsudo](https://github.com/makethunder/awsudo/) is heavily used where I work and it constantly causes pain, the CLI has a couple of issues:
1. **Distribution** - It was written in Python, which makes it difficult to distribute, also doens't have a homebrew formula
1. **Dependencies** - It [locks the aws-cli version](https://github.com/makethunder/awsudo/issues/7), and [depends on code of the CLI itself](https://github.com/makethunder/awsudo/blob/d5800bc4a9785d179c678605d0ae5bf4e28f5205/awsudo/config.py#L1)
1. **Versioning** - It [doesn't have versions whatsoever](https://github.com/makethunder/awsudo/releases)
1. **Bugs** - It has a couple of bugs, e.g.: [you can't pass `AWS_*` like variables to your command](https://github.com/makethunder/awsudo/issues/14)

The [official solution](https://docs.aws.amazon.com/cli/latest/userguide/cli-roles.html#cli-roles-cache) from AWS is not really great either:
1. Stateful - Once you assume you stick with it until it expires or you switch
1. CLI centric - Hard to share the credentials and switch quickly if you are not using their CLI

## Usage

```
awsudo - sudo-like behavior for role assumed access on AWS accounts

USAGE:
    awsudo [OPTIONS] --user <user> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>    Sets a custom config file other than ~/.aws/credentials
    -u, --user <user>      Set the AWS profile name based on the config file
```

Thanks [clap](https://github.com/clap-rs/clap) for that.

For example, to get all of the S3 buckets of the **production** account:

```shell
awsudo -u production aws s3 ls
```

Runnin a executable that needs AWS credentials:

```shell
awsudo -u staging delete_s3_buckets "s3://staging-acc/foo"
```

#### Aliases

This is not required, but interesting:

```shell
alias awss='awsudo -u staging'
alias awso='awsudo -u operations'
alias awsp='awsudo -u production'
```

Then after:

```shell
awss [subcomand]
```

## Workflow

This is how the CLI works under the hood, for transparency and organization purposes.

**Notice** some features of the workflow are a work in progress.

(click to see the large version)

<img src="docs/workflow.png" width="10%">

# Contributing
> Help us to improve the codebase

Found a bug? Have a suggestion? Please [open an issue](https://github.com/marceloboeira/awsudo/issues/new).

Want to contribute with code?

## Developers

1. Star the project.
2. Open or find an issue [here](https://github.com/marceloboeira/awsudo/issues)
3. Fork it (https://github.com/marceloboeira/awsudo/fork)
4. Create your feature branch (git checkout -b feature/awesome-parrot)
5. Commit your changes
6. Push to the branch (git push origin feature/awesome-parrot)
7. Create a new Pull Request

## Build & Run
> Available commands

* `make build` - Build the `/target/debug/awsudo`.
* `make build_release` - Build the optmized `/target/release/awsudo`.
* `make install` - Use cargo insttall.
* `make test` - Run the tests.
* `make format` - Format the code following rust standards.
* `make setup_docs` - One time setup of dependencies for docs.
* `make docs` - Generate documentation diagrams.
