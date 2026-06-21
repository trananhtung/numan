# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-21

### Added

- Initial release.
- `compact` — short business notation (`1.2K`, `3.4M`, `1.2B`).
- `word` — short-scale words (`1.2 million`, `7 billion`).
- `metric` — SI metric prefixes (`1.2 k`, `2.2 M`).
- `ordinal` — English ordinal suffixes (`1st`, `22nd`, `113th`).
- `parse` — parse humanized strings back into `f64`.
- `Formatter` builder with `precision`, `trim_zeros`, `space` and `sign_plus`.
- `#![no_std]` support (requires `alloc`); zero dependencies.

[0.1.0]: https://github.com/trananhtung/numan/releases/tag/v0.1.0
