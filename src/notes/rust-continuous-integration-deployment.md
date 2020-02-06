# Continuous Integration and Deployment in Rust

Cargo can be used for static analysis and releasing. For example, with something like the following:

## Run CI when pushing to branches for pull requeests

```yaml
name: Pull request
on:
  push:
    branches-ignore:
      - master
jobs:
  verify:
    runs-on: ubuntu-latest
    steps:
    - name: Checkout
      uses: actions/checkout@v1
    - name: Check
      run: cargo check
    - name: Test
      run: cargo test
    - name: Lint
      run: cargo clippy --all-targets -- -D warnings
    - name: Format
      run: cargo fmt -- --check
    - name: Publish
      run: cargo publish --dry-run
```

## Run CI and CD when merging pull requests to master

```yaml
name: Merge
on:
  push:
    branches:
      - master
jobs:
  verify:
    runs-on: ubuntu-latest
    steps:
    - name: Checkout
      uses: actions/checkout@v1
    - name: Check
      run: cargo check
    - name: Test
      run: cargo test
    - name: Lint
      run: cargo clippy --all-targets -- -D warnings
    - name: Format
      run: cargo fmt -- --check
  executable:
    runs-on: ubuntu-latest
    needs: [verify]
    steps:
    - name: Checkout
      uses: actions/checkout@v1
    - name: Login
      run: cargo login ${{ secrets.CRATE_REGISTRY_PAT }}
    - name: Publish
      if: success()
      run: cargo publish
```
