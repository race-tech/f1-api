[![build & test](https://github.com/f1-tech/api/blob/master/.github/workflows/rust.yml/badge.svg)](https://github.com/f1-tech/api/blob/master/.github/workflows/rust.yml)
[![circleci](https://img.shields.io/circleci/build/gh/f1-tech/api?logo=circleci&logoColor=white&label=circleci)](https://circleci.com/gh/f1-tech/api)
[![license](https://img.shields.io/github/license/f1-tech/api)](https://www.gnu.org/licenses/gpl-3.0.en.html)
[![rust report card](https://rust-reportcard.xuri.me/badge/github.com/f1-tech/api)](https://rust-reportcard.xuri.me/report/github.com/f1-tech/api)

# F1 API

This is a Fomula One API build in Rust and currently under development. The goal is to provide a simple and easy to use API to access the ergast database and in the end, provide a new maintained database after the 2024 season.

## Prerequisites

To build and run the API, you need to have the following tools installed on your system:

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)

## Development

### Using Docker

To run the API locally, follow these steps:

1. Clone the repository: `git clone https://github.com/f1-tech/api.git`
2. Run the project using the docker compose file: `docker-compose up -d --build`
