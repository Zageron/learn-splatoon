# Learn Splatoon

[![dependency status](https://deps.rs/repo/github/zageron/learn-splatoon/status.svg)](https://deps.rs/repo/github/zageron/learn-splatoon)
[![Rust](https://github.com/Zageron/learn-splatoon/actions/workflows/build.yml/badge.svg?event=pull_request)](https://github.com/Zageron/learn-splatoon/actions/workflows/build.yml)

Personal implementation of the SM-2 SRS algorithm utilized to create a spaced reptition system to learn Splatoon callouts.

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
act push --secret-file ".env"
```
