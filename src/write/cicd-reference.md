# CI/CD reference

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
        run: some_compile

  test:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Test
        run: some_test

  lint:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Lint
        run: some_lint

  format:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Format
        run: some_format

  build:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Build
        run: some_build
      - name: Audit
        if: success()
        uses: treosh/lighthouse-ci-action@v3
        with:
          configPath: "./.github/configuration/audit-local.json"
          temporaryPublicStorage: true
      - name: Links
        if: success()
        uses: peter-evans/link-checker@v1
        id: links
        with:
          args: --document-root ./build --verbose --recursive *
      - name: Exit
        run: exit ${{ steps.links.outputs.exit_code }}
```

In `.github/configuration/audit-local.json`:

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
        run: some_build
      - name: Deploy
        if: success()
        uses: some_deploy
```

## Setup scheduled checks

Examples:

```yaml
name: Schedule

on:
  schedule:
    - cron: "0 0 * * *"

jobs:
  e2e:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: E2E
        run: some_e2e
      - name: Audit
        uses: treosh/lighthouse-ci-action@v3
        with:
          configPath: "./.github/configuration/audit-production.json"
          temporaryPublicStorage: true
          runs: 3
          urls: |
            https://someproject.com
            https://someproject.com/someroute/
```

In `.github/configuration/audit-production.json`:

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
