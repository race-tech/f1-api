[![build & test](https://github.com/purpl-sectr/api/blob/master/.github/workflows/rust.yml/badge.svg)](https://github.com/purpl-sectr/api/blob/master/.github/workflows/rust.yml)
[![circleci](https://img.shields.io/circleci/build/gh/purpl-sectr/api?logo=circleci&logoColor=white&label=circleci)](https://circleci.com/gh/purpl-sectr/api)
[![license](https://img.shields.io/github/license/purpl-sectr/api)](https://www.gnu.org/licenses/gpl-3.0.en.html)
[![rust report card](https://rust-reportcard.xuri.me/badge/github.com/purpl-sectr/api)](https://rust-reportcard.xuri.me/report/github.com/purpl-sectr/api)

# PurpleSector

This is a Fomula API build in Rust and currently under development. The goal is to provide a simple and easy to use API to access the ergast database and in the end, provide a new maintained database after the 2024 season.

## Prerequisites

To build and run the PurpleSector API, you need to have the following tools installed on your system:

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)

## Development

To run the PurpleSector API locally, follow these steps:

1. Clone the repository: `git clone https://github.com/thibault-cne/purple-sector.git`
2. Run the database: `docker-compose up -d database dragonfly`
3. Run the project: `cargo run --release`
