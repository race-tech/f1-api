[![build & test](https://github.com/thibault-cne/rust-race-engine/actions/workflows/rust.yml/badge.svg)](https://github.com/thibault-cne/rust-race-engine/actions/workflows/rust.yml)
[![circleci](https://img.shields.io/circleci/build/gh/thibault-cne/rust-race-engine?logo=circleci&logoColor=white&label=circleci)](https://circleci.com/gh/thibault-cne/rust-race-engine)
[![appveyor](https://img.shields.io/appveyor/build/thibault-cne/rust-race-engine?logo=appveyor&logoColor=white&label=appveyor)](https://ci.appveyor.com/project/thibault-cne/rust-race-engine)
[![license](https://img.shields.io/github/license/thibault-cne/rust-race-engine)](https://www.gnu.org/licenses/gpl-3.0.en.html)
[![rust report card](https://rust-reportcard.xuri.me/badge/github.com/thibault-cne/rust-race-engine)](https://rust-reportcard.xuri.me/report/github.com/thibault-cne/rust-race-engine)

# Rust race engine

This is a Fomula API build in Rust and currently under development. The goal is to provide a simple and easy to use API to access the ergast database and in the end, provide a new maintained database after the 2024 season.

## Project Roadmap

- [x] Create all the basic API endpoints to access the ergast DB
- [ ] Create a complete swagger documentation
- [ ] Create an automated tool to update the DB in prevision of the ergast deprecated DB for next season
- [ ] Create a new DB for the F2 championship (maybe even F3 and F1 academy as well as WEC or other championships)
- [ ] Add a predictor to the API (an LLM model that will try to predict upcoming races results)
