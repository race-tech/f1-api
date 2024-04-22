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
