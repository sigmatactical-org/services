# sigma-services

[![CI](https://github.com/sigmatactical-org/services/actions/workflows/ci.yml/badge.svg)](https://github.com/sigmatactical-org/services/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![MSRV](https://img.shields.io/badge/MSRV-1.97.0-blue.svg)](https://www.rust-lang.org)

Public site for Sigma Tactical Group **professional services**: vehicle maintenance, consulting, and research & development.

Repository: https://github.com/sigmatactical-org/services

## Features

Service pages are Markdown files under `content/` with optional YAML front matter:

```yaml
---
title: Consulting
summary: One-line card blurb for the index page
order: 2
---
```

Pages are embedded at compile time and served at `/service/{slug}`.

## Configuration

| Variable | Purpose |
| --- | --- |
| `PORT` | Listen port (default `8080`) |
| `SERVICES_PUBLIC_BASE_URL` | Public base URL of this site (default `http://127.0.0.1:8080/`) |
| `SERVICES_IDENTITY_PUBLIC_URL` | Identity BFF base URL; its origin is also added to the CSP `connect-src` (default `http://127.0.0.1:3000/`) |
| `SERVICES_CART_PUBLIC_URL` | Cart service base URL for navbar links (default `http://127.0.0.1:8084/`) |
| `SERVICES_CONTACT_PUBLIC_URL` | Contact service base URL for inquiry buttons (default `http://127.0.0.1:8083/`) |

## Development

```bash
cargo run
# http://127.0.0.1:8080/
```

### Shared crates

`sigma-theme` and `sigma-pg` are pinned git dependencies, so a
fresh clone builds with nothing but `cargo`: the revision in `Cargo.toml` is
fetched, and `build.rs` writes the `askama.toml` that points at sigma-theme's
templates wherever Cargo put them.

When one of those crates is checked out beside this repo and you are editing it,
link the checkouts so your edits are picked up without a push:

```bash
./scripts/prepare-local.sh
```

That writes `[patch]` entries into `.cargo/config.toml` (gitignored) for the
crates it finds and leaves the rest on their pinned revision; it prints what it
linked. Undo by deleting the file. Note that building against a linked checkout
rewrites `Cargo.lock` into path form — don't commit that; `platform`'s
`scripts/relock.sh` restores the git-resolved lockfile CI expects.

Bumping a shared crate is `platform/scripts/pin-shared-revs.sh <crate>` after
that crate is pushed, which updates every consumer's pin at once.

### Platform

Kubernetes manifests live in [platform](https://github.com/sigmatactical-org/platform) under `it/platform/services/services/`. Dev ingress: `http://services.sigma.localtest.me:30080/`.

## Docker

```bash
./scripts/docker-build.sh
docker build -f Dockerfile build/image -t sigma-services:local
```

## Brand & artwork

© Sigma Tactical Group. **All rights reserved.**

The Sigma Tactical Group name, logos, marks, artwork, and visual identity are **proprietary**. They are not covered by this repository's source-code license. See [BRANDING.md](BRANDING.md).

## License

MIT OR Apache-2.0 for **source code** only. Branding remains proprietary.
