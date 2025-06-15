# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

Here’s what to put in your `CHANGELOG.md` for the next patch release `v0.1.1` — assuming you just fixed release packaging and install script issues:

---

### ✅ `CHANGELOG.md` – Entry for `v0.1.1`

```markdown
## v0.1.1 - 2025-06-15

### Fixed
-  Resolved broken tarball packaging: release archive now includes correct folder structure:
```
rust-token-fetch-v0.1.1/
├── linux/rust-token-fetch
└── macos/rust-token-fetch
```
- Fixed GitHub Actions `gh-release` job to properly use `upload-artifact`/`download-artifact`.
- `install.sh` now installs the correct binary based on `uname -s` and `uname -m` detection.
- Verified install via:
```bash
curl -sL https://raw.githubusercontent.com/ayuspoudel/rust-gh-token-fetch/main/install.sh | bash
```

### Changed

* Restructured `gh-release` logic to decouple platform builds and centralize packaging.
* Improved README and CHANGELOG for clarity and install instructions.


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

