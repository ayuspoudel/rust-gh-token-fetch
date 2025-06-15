# rust-token-fetch

A blazing fast, zero-dependency CLI to generate GitHub App installation tokens. No config files. Just `--app-id`, `--installation-id`, and your private key.

---

## ⚡ Usage

```bash
# Option 1: Using private key path
./gh-token --app-id 1331774 \
           --installation-id 68637543 \
           --private-key-path private-key.pem

# Option 2: Using .env (recommended for CI)
GITHUB_APP_ID=1331774
GITHUB_INSTALLATION_ID=68637543
GITHUB_APP_PRIVATE_KEY='-----BEGIN RSA PRIVATE KEY-----...'
./gh-token
