# Learn Splatoon

[![dependency status](https://deps.rs/repo/github/zageron/learn-splatoon/status.svg)](https://deps.rs/repo/github/zageron/learn-splatoon)

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
