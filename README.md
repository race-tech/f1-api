[![build & test](https://github.com/thibault-cne/purple-sector/actions/workflows/rust.yml/badge.svg)](https://github.com/thibault-cne/purple-sector/actions/workflows/rust.yml)
[![circleci](https://img.shields.io/circleci/build/gh/thibault-cne/purple-sector?logo=circleci&logoColor=white&label=circleci)](https://circleci.com/gh/thibault-cne/purple-sector)
[![license](https://img.shields.io/github/license/thibault-cne/purple-sector)](https://www.gnu.org/licenses/gpl-3.0.en.html)
[![rust report card](https://rust-reportcard.xuri.me/badge/github.com/thibault-cne/purple-sector)](https://rust-reportcard.xuri.me/report/github.com/thibault-cne/purple-sector)

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
