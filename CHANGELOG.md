# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.2.0](https://gitlab.com/21analytics/lei/-/tags/0.2.0) - 2023-06-19

- Added a new `RegistrationAuthority` type.
- Updated the registration authority list to [v1.7](https://www.gleif.org/about-lei/code-lists/gleif-registration-authorities-list/2022-03-23_ra_list_v1.7.xlsx).
- Pinned the minimum required `diesel` version to `2.1.0`. and fixed
  the (feature-flag protected) breaking changes.

## [0.1.2](https://gitlab.com/21analytics/lei/-/tags/0.1.2) - 2023-06-12

- Fix the docs.rs build by activating the required nightly feature.

## [0.1.1](https://gitlab.com/21analytics/lei/-/tags/0.1.1) - 2023-06-12

- Fix Diesel build issue by pinning down Diesel version.

## 0.1.0

- Initial release of this crate.
