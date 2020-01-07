# Dependency management in Rust

## Add a dependency

- `some_library = "x.y.z"` manually added in `[dependencies]` section of `Cargo.toml`
- From [crates.io](http://crates.io/)
- Updates `Cargo.lock` during the next compile

## Use a dependency

- `use some_library::some_module`
- Brings the last item after the `::` into scope on top of the minimal prelude

## Upgrade a dependency

- Manually change in `[dependencies]` section of `Cargo.toml` to a different version
- Updates `Cargo.lock` during the next compile
