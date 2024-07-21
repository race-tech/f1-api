[![build & test](https://img.shields.io/github/actions/workflow/status/race-tech/f1-api/rust.yml?logo=github)](https://github.com/race-tech/f1-api/blob/master/.github/workflows/rust.yml)
[![circleci](https://img.shields.io/circleci/build/gh/race-tech/f1-api?logo=circleci&logoColor=white&label=circleci)](https://circleci.com/gh/race-tech/f1-api)
[![license](https://img.shields.io/github/license/race-tech/f1-api)](https://www.gnu.org/licenses/gpl-3.0.en.html)
[![rust report card](https://rust-reportcard.xuri.me/badge/github.com/race-tech/f1-api)](https://rust-reportcard.xuri.me/report/github.com/race-tech/f1-api)

# F1 API

This is a Fomula One API build in Rust and currently under development. The goal is to provide a simple and easy to use API to access the ergast database and in the end, provide a new maintained database after the 2024 season.

## Prerequisites

To build and run the API, you need to have the following tools installed on your system:

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)

## Development

### Using Docker

To run the API locally, follow these steps:

1. Clone the repository: `git clone https://github.com/race-tech/f1-api.git`
2. Run the project using the docker compose file: `docker-compose -f docker-compose.dev.yml up -d --build`

## Configuration

You can configure the API using a yaml file. This file should be named `config.yaml` and should be placed in the root of the project. The following configuration options are available:

```yaml
port: 8000

database:
  name: f1db
  hostname: database
  port: 3306
  user: user
  password: password

middlewares:
  - graphiql:
      enabled: true
      route: /
```

You can also change the file name and location by setting the `F1_API_CONFIG` environment variable.

## License

This project is licensed under the [GPL-3.0 License](https://github.com/race-tech/f1-api/blob/master/LICENSE).
