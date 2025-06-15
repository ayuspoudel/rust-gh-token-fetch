# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## v0.1.0 - 2025-06-15

### Added
- Initial release of `rust-token-fetch` binary.
- GitHub App JWT generation using RSA private key.
- Installation access token fetch for GitHub App.
- Auto-writes token into `github.auto.tfvars.json`.
- `.env` support via `dotenvy`.
- CLI powered by `clap` with `--app-id`, `--installation-id`, `--private-key`, and `--output`.
- Supports `--private-key` and `--private-key-path` options.
- Release workflow for Linux binaries (`.tar.gz`) with GitHub Actions.

