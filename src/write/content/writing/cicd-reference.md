+++
title = "CI/CD reference"
weight = 2
+++

_My reference sheet for automation with CI/CD._

## Setup continuous integration with a protected branch and merge checks

Examples:

```yaml
name: Continuous Integration

on:
  push:

jobs:
  compile:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Compile
        run: cargo check
      - name: Build
        run: cargo run --release

  test:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Test
        run: cargo test

  lint:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Lint
        run: cargo clippy --all-targets -- -D warnings
      - name: General
        uses: docker://github/super-linter:v2.0.0

  format:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Format
        run: cargo fmt -- --check
      - name: General
        run: npx prettier --check . --ignore-path ./.gitignore

  links:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Links
        uses: peter-evans/link-checker@v1
        id: links
        with:
          args: --document-root ./build --verbose --recursive *
      - name: Exit
        run: exit ${{ steps.links.outputs.exit_code }}

  audit:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Audit
        uses: treosh/lighthouse-ci-action@v3
        with:
          configPath: "./audit-local.json"
          temporaryPublicStorage: true
```

`audit-local.json`

```json
{
  "ci": {
    "collect": {
      "staticDistDir": "./build",
      "settings": { "skipAudits": ["uses-http2"] }
    },
    "assert": {
      "assertions": {
        "categories:performance": ["error", { "minScore": 0.9 }],
        "categories:accessibility": ["error", { "minScore": 0.9 }],
        "categories:best-practices": ["error", { "minScore": 0.9 }],
        "categories:seo": ["error", { "minScore": 0.9 }],
        "uses-http2": "off"
      }
    }
  }
}
```

## Setup continuous deployment

Examples:

```yaml
name: Continuous Deployment

on:
  push:
    branches:
      - main

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Build
        run: cargo run --release
      - name: Deploy
        if: success()
        uses: someprovider
```

## Setup scheduled checks

Examples:

```yaml
name: Schedule

on:
  schedule:
    - cron: "0 * * * *"

jobs:
  health:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Health
        run: |
          home_route="$(curl https://someproject.com)"
          if [[ $articles_route != *"Some content."* ]]; then
            echo "ERROR: The home route is not working as expected!";
            exit 1;
          fi
          some_route="$(curl https://someproject.com/someroute/)"
          if [[ $about_route != *"Some content."* ]]; then
            echo "ERROR: The some route is not working as expected!";
            exit 1;
          fi

  audit:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Audit
        uses: treosh/lighthouse-ci-action@v3
        with:
          configPath: "./audit-production.json"
          temporaryPublicStorage: true
          runs: 3
          urls: |
            https://someproject.com
            https://someproject.com/someroute/
```

`audit-production.json`

```json
{
  "ci": {
    "assert": {
      "assertions": {
        "categories:performance": ["error", { "minScore": 0.9 }],
        "categories:accessibility": ["error", { "minScore": 0.9 }],
        "categories:best-practices": ["error", { "minScore": 0.9 }],
        "categories:seo": ["error", { "minScore": 0.9 }]
      }
    }
  }
}
```
