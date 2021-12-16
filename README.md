# Learn Splatoon

[![dependencies](https://deps.rs/repo/github/zageron/learn-splatoon/status.svg)](https://deps.rs/repo/github/zageron/learn-splatoon)
[![CI](https://github.com/Zageron/learn-splatoon/actions/workflows/validate.yml/badge.svg)](https://github.com/Zageron/learn-splatoon/actions/workflows/validate.yml)
![CI](https://img.shields.io/github/workflow/status/Zageron/learn-splatoon/CI?style=flat-square)
[![rustc](https://img.shields.io/badge/rustc-1.57-lightgray.svg?style=flat-square)](https://www.rust-lang.org/)
[![discord](https://img.shields.io/discord/921110926751584346?label=discord&color=7289DA&logo=discord&logoColor=white&style=flat-square)](https://discord.com/invite/g4uHt46U)

:x: __Note: This repository and project is incomplete and not functional yet, you wont find much other than data.__

Spaced Repetition learning platform for Splatoon 2 callouts, weapons, specials, and subs.

## Development Environment

### Dependency Update

```bash
cargo upgrade --allow-prerelease ;; cargo update ;; cargo build ;; cargo build --release
```

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
