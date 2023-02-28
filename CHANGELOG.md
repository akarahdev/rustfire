# Changelog
All notable changes will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) after version 1.0.0. Until 1.0.0 is released, assume each version is not backwards compatible unless specified.

## Roadmap
### Guaranteed
- Support for all values and all codeblocks. Once this is added, adding in new blocks will be super easy to do. Could even be automated?
### Ideas
- Removing the need for `scan.py` by having functionality built-in.
- Safety checks on certain codeblocks to make sure input is valid, like a linter for DF code.
## 0.3.0
### Added
- Added the `df!` macro to streamline building codelines. See `README.md` for more information.
- Arguments are no longer passed in as `std::str`. Instead, they are passed in as `rustfire::item::Item`.
- More events and codeblocks added.
- Multiple item inputs are now allowed instead of just one.
- You can now pass in parameters to functions using `function!(CallParams ...`! Eventually this will be mertged with the primary `function!(Call ...` 
- Added atleast one copy of **every** codeblock.
- Added variable support. Game values are upcoming.
## 0.2.0 - 2/26/23
### Added
- Recode support, now it exports directly to Recode! However, it currently relies on `send.py` to work.
- All code segments should now be started with `build_set!("template name here");`
- Added some code actions.
## 0.1.1 - 2/26/23
### Added
- Fixes to `Cargo.toml` and `README.md`
## 0.1.0 - 2/25/23
### Added
- Changelog file
- Github Repo
- Basic systems (items, codeblocks, player events, player actions)
