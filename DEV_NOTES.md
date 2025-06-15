# Dev Notes

```bash
git clone https://github.com/ayuspoudel/rust-token-fetch
cd rust-token-fetch
cargo build --release
```

To run:

```bash
cp .env.example .env  # edit your app ID, installation ID, PEM
cargo run
```
---

##  Project Structure

```bash
src/
├── main.rs             # CLI logic, entrypoint
├── fetch_token.rs      # Token request + write to tfvars
└── jwt.rs              # JWT generation (RS256)
```

No dynamic config. All state is passed via CLI or `.env`.

---

## Rules

* No `unwrap()` unless absolutely safe
* No third-party crates unless justified
* No async — this app is fast enough synchronously
* Always check token error status (`401`, `403`, etc.)

Open an issue before you add any new features.


# CODE DESIGN

# Code Design – `rust-token-fetch`

This app is built with one goal: make GitHub App token generation bulletproof and portable.


## Design Principles

- Secrets are passed via environment or CLI only
- No config files, no magic — explicit args always win
- Token is generated in under 200ms (no async needed)
- Stateless: you run it → it prints → done

---

## JWT Generation

- Uses RS256 via `jsonwebtoken` crate
- `iat` = now
- `exp` = now + 10 mins
- `iss` = GitHub App ID

```rust
Claims {
  iat: now,
  exp: now + 600,
  iss: app_id.to_string()
}
```


## Modules

* `main.rs`: CLI, env loader, dispatcher
* `fetch_token.rs`: Talks to GitHub API, validates response, writes `.tfvars`
* `jwt.rs`: Generates signed JWT using RS256


## Why No Config

You don’t need `config.toml` or `repo-mapping.yaml`. Just pass what you want to use. Simpler, safer, CI-friendly.


## Future Enhancements

* `--jwt-only` mode
* JSON output (`--json`)
* Homebrew install formula