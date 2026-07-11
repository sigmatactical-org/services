# sigma-services

Public site for Sigma Tactical Group **professional services**: vehicle maintenance, consulting, and research & development.

Repository: https://github.com/sigmatactical-org/services

## Content

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
| `SERVICES_CONTACT_PUBLIC_URL` | Contact service base URL for inquiry buttons (default `http://127.0.0.1:8083/`) |

## Local development

```bash
./scripts/prepare-local.sh
cargo run
# http://127.0.0.1:8080/
```

## Docker

```bash
./scripts/docker-build.sh
docker build -f Dockerfile build/image -t sigma-services:local
```

## Platform

Kubernetes manifests live in [platform](https://github.com/sigmatactical-org/platform) under `it/platform/services/services/`. Dev ingress: `http://services.sigma.localtest.me:30080/`.

## Brand & artwork

© Sigma Tactical Group. **All rights reserved.**

The Sigma Tactical Group name, logos, marks, artwork, and visual identity are **proprietary**. They are not covered by this repository's source-code license. See [BRANDING.md](BRANDING.md).

## License

MIT OR Apache-2.0 for **source code** only. Branding remains proprietary.
