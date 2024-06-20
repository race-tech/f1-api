# Version v0.2.0 (Jun 20, 2024)

## Major features

- Rename the project to F1 API
- Changed the project structure to use a microservice architecture
- Changed the API to use a GraphQL interface
- Removed the rate limiter and cache from the API (this will be added in the api gateway)

## Minor features

- Added a configuration guide to the README
- Added issue and pull request templates
- Added dependabot to the project
- Added the port configuration to the configuration file
- Dumped unused dependencies (`proc_macro2`, `quote` and `syn`)

# Version v0.1.2 (Apr 21, 2024)

This is the first release of the PurpleSector API. This version is not complete but should be stable enough to be used in production.

## Major features

- Added a rate limiter to the API backed by a dragonfly database
- Support to access all tables of the ergast database
- Added a configuration file to the API (see [config](config.yml) for more details)
- Dumped the `rocket` web framework in favor of `axum`. This change was made to improve the performance of the API and reduce the number of dependencies
- Stabilized the API by adding more tests and enhancing the error handling

# Version v0.1.1-alpha (Apr 14, 2024)

## Warning

This is a pre-release version of the PurpleSector API. This version is not yet stable and should not be used in production.

## Major features

- Added a rate limiter to the API backed by a dragonfly database

# Version v0.1.0-alpha (Apr 13, 2024)

## Warning

This is a pre-release version of the PurpleSector API. This version is not yet stable and should not be used in production.

## Major features

- Support to access all tables of the ergast database

## Minor features

- Added initial documentation on the project
