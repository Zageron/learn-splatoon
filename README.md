# Learn Splatoon

[![dependency status](https://deps.rs/repo/github/zageron/learn-splatoon/status.svg)](https://deps.rs/repo/github/zageron/learn-splatoon)
[![Rust](https://github.com/Zageron/learn-splatoon/actions/workflows/build.yml/badge.svg)](https://github.com/Zageron/learn-splatoon/actions/workflows/build.yml)

:x: __Note: This repository and project is incomplete and not functional yet, you wont find much other than data.__

Spaced Repetition learning platform for Splatoon 2 callouts, weapons, specials, and subs.

## Development Environment

### Ory Proxy

VSC Task should start this automatically.

```bash
ory proxy "http://localhost" --sdk-url "https://gallant-leakey-n498g2rdpa.projects.oryapis.com"
```

## Deploy Testing

```bash
docker build -t learn-splatoon .
docker run -p 8081:8081 --env-file .env -ti learn-splatoon
```

## Github Action Testing

```bash
act push --env-file ".env" --reuse
```
