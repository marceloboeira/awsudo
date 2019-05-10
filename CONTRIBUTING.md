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
* `make release` - Build and generate the release file for the given version.
* `make install` - Use cargo install.
* `make test` - Run the tests.
* `make test_watcher` - Run the tests under a watcher.
* `make docker_test_watcher` - Run the tests under a watcher on Docker (to ensure linux compatibility).
* `make format` - Format the code following rust standards.
* `make setup_docs` - One time setup of dependencies for docs.
* `make docs` - Generate documentation diagrams.
