# hltvapi.rs

[![Build](https://github.com/f4bio/hltvapi.rs/actions/workflows/build.yml/badge.svg)](https://github.com/f4bio/hltvapi.rs/actions/workflows/build.yml)
[![Containerize](https://github.com/f4bio/hltvapi.rs/actions/workflows/containerize.yml/badge.svg)](https://github.com/f4bio/hltvapi.rs/actions/workflows/containerize.yml)
[![codecov](https://codecov.io/gh/f4bio/hltvapi.rs/branch/main/graph/badge.svg?token=RAGGFAZE0Y)](https://codecov.io/gh/f4bio/hltvapi.rs)
[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white)](https://github.com/pre-commit/pre-commit)

## table of contents

<!-- START doctoc -->

<!-- END doctoc -->

## helpful links

- [https://cloudmaker.dev/how-to-create-a-rest-api-in-rust/](https://cloudmaker.dev/how-to-create-a-rest-api-in-rust/)
- [reqwests](https://stackoverflow.com/a/51047786)
- [rust-actix-microservice-auth-example](https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/)

## Scripts

### bump version

- openapi.yaml: `yq e -i '.info.version = "0.2.7"' openapi.yaml`
- Cargo.toml: `cargo bump patch`
- package.json: `npm version patch`

## Documentation

### redoc

```bash
redoc-cli bundle ./docs/openapi.yaml \
  --template ./docs/template.hbs \
  --output ./static/docs.html
```

## Data

### fixtures

see [full example (all)](./tests/scraped.fixtures_all.example.html)
and [full example (top tier)](./tests/scraped.fixtures_toptier.example.html)

### news

see [full example (all)](./tests/scraped.news_all.example.html)
and [specific item](./tests/scraped.news_item.example.html)

## Tests

### with `newman`

see
newman's [getting started](https://learning.postman.com/docs/running-collections/using-newman-cli/command-line-integration-with-newman/#getting-started)

```bash
npx newman run ./docs/hltvapi.postman_collection.json
```

## Environment

### App Key

Generate with your favorite tool, for example with [apg](https://github.com/buzo-ffm/apg): `$ apg -n 5 -m 40 -t`

## CI

### GitHub Actions

Run locally with [act](https://github.com/nektos/act):

```bash
act \
  --secret GITHUB_TOKEN=... \
  --secret CODECOV_TOKEN=... \
  --secret AWS_ACCESS_KEY_ID=... \
  --secret AWS_SECRET_ACCESS_KEY=... \
  --secret DIGITALOCEAN_ACCESS_TOKEN=...
```
