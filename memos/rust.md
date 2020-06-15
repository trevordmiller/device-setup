# Rust

Standard Rust usage.

## View documentation

```shell
rustup doc
```

## Scaffold a new executable project

```shell
cargo new some_executable
```

## Scaffold a new library project

```shell
cargo new some_library --lib
```

## Compile an executable in development mode

```shell
cargo build
```

## Run a development mode executable

```shell
./target/debug/some_executable
```

## Compile an executable in production mode

```shell
cargo build --release
```

## Run a production mode executable

```shell
./target/release/some_executable
```

## Compile the executable in development mode then run the executable

```shell
cargo run
```

## Compile the executable in production mode then run the executable

```shell
cargo run --release
```

## Check for compilation errors

```shell
cargo check
```

## Get more information on a compilation error

```shell
rustc --explain some_error_id
```

## Include a backtrace on a compilation error

```shell
RUST_BACKTRACE=1 cargo ...
```

## Fix compilation errors that can be fixed automatically

```shell
cargo fix
```

## Upgrade syntax to a new edition (in Cargo.toml)

```shell
cargo fix --edition
```

## Test logic

```shell
cargo test
```

## Lint for common issues

```shell
cargo clippy
```

## Format source code

```shell
cargo fmt
```

## Generate project documentation

```shell
cargo doc
```

## View project documentation

```shell
cargo doc --open
```

## Share an executable

```shell
cargo build --release
```

- Outputs to `./target/release/some_project`
- Default target is the host architecture

## Setup continuous integration

Cargo can be used for static analysis. For example, with something like the following:

```yaml
on:
  push:
  schedule:
    - cron: "0 0 * * *"

jobs:
  compile:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Compile
        run: cargo check

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

  format:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Format
        run: cargo fmt -- --check
```

## Add a dependency

- `some_library = "x.y.z"` manually added in `[dependencies]` section of `Cargo.toml`
- From `crates.io`
- Updates `Cargo.lock` during the next compile (`cargo build`, `cargo run`, `cargo check`, `cargo test`, etc.)

## Upgrade all dependencies to the latest patch versions

```shell
cargo update
```

## Upgrade a dependency to newer minor/major versions

- Manually change in `[dependencies]` section of `Cargo.toml` to a different version
- Updates `Cargo.lock` during the next compile

## Bring a library into scope

```rust
use some_library::some_module;
```

- Brings the last item after the `::` into scope on top of the minimal prelude

## Create a public facing contract

```rust
pub ...
```

## Add public facing documentation (with integration tests)

````rust
/// Some documentation comment.
///
/// # Examples
///
/// ```
/// assert_eq!(some_library::some_module::some_function(some_input), some_output);
/// ```
pub fn some_function ...
````

## Add unit tests

```rust
fn some_function ...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_test() {
        assert_eq!(some_function(some_input), some_output);
    }

    #[test]
    fn another_test() {
        assert_eq!(some_function(some_input), some_output);
    }
}
```

## Debug an expression

```rust
dbg!(some_expression)
```

## Print to the terminal

```rust
println!("Some output")
```

## Print to the terminal with values

```rust
println!("Some output with {} and {}", some_value, another_value, ...)
```

## Print to the terminal with a value for debugging

```rust
println!("Some output with {:?}", some_value)
```

## Handle errors that have a `Result` type

```rust
match some_result {
    Ok(some_value) => // use some_value,
    Err(some_error) => // handle error,
}
```

## Handle errors that have an `Option` type

```rust
match some_option {
    Some(some_value) => // use some_value,
    None => // handle error,
}
```

## Crash (panic) when an error occurs

```rust
some_thing_that_can_fail.unwrap()
some_thing_that_can_fail.expect("Some error message")
```

## Crash (panic) when an error occurs with a custom error message

```rust
some_thing_that_can_fail.expect("Some error message")
```

## Add internal comments

```rust
// Some comment
```

## Store values in read-only variables

```rust
let some_variable = some_value
```

## Store values in writable variables

```rust
let mut some_variable = some_value
```

## Store values to be shared across multiple parts of a program (constants)

```rust
const SOME_CONST: some_type = some_value
```

## Refer to a value without taking ownership of it (references)

```rust
&some_value
```

## Call a macro

```rust
some_macro!(some_argument, another_argument, ...)
```

## Call a function

```rust
some_function(some_argument, another_argument, ...)
```

## Define a function

```rust
fn some_function(some_argument: some_argument_type, another_argument: another_argument_type, ...) -> some_return_type {
    // Body can contains statement(s) and/or a return expression
    some_statement;
    another_statement;
    // An expression without a semicolon is what is returned from the function body
    some_return_expression
}
```

## Define the entry point for an executable project

```rust
fn main() {
    ...
}
```

## Group values of different types (tuple)

```rust
(some_value, another_value, ...)
```

## Pull out tuple values (destructuring)

```rust
let some_tuple = (some_value, another_value, ...);
let (some_variable, another_variable, ...) = some_tuple;
```

## Add type annotations when types cannot be inferred on values

```rust
some_value: some_type
```

## Annotate a type for a number without a decimal (integer types)

```rust
u/isome_length
```

- Like `u32`, `i32`, `u64`, etc. If can be negative needs signed (`i`), if only positive can be unsigned (`u`).

Examples:

```rust
let answer: u32 = 42;
```

## Annotate a type for a number with a decimal (floating-point types)

```rust
f{length}
```

Examples:

```rust
let answer: f64 = 42.0;
```

## Annotate a type for two possible values / true or false (boolean type)

```rust
bool
```

Examples:

```rust
let is_cool = true;
```

# Annotate a type for a unicode value / letter (character type)

```rust
char
```

Examples:

```rust
let laughing = 😂;
```

## Annotate a type for grouped values with a variety of types (tuple type)

```rust
(some_type, another_type, ...)
```

Examples:

```rust
let person = ("Jane", "Doe", 42)
```

## Annotate a type for a collection of values with the same type (vector, allocated on the heap)

```rust
Vec<some_type>
```

Examples:

```rust
let users = vec![1, 2, 3]
```

## Annotate a type for a collection of a fixed number of values with the same type (array, pushed onto / popped off the stack)

```rust
[some_type; some_size]
```

Examples:

```rust
let sizes = ["small", "medium", "large"];
```

## Create a literal str

```rust
"Some string"
```

## Create a literal char

```rust
'😂'
```

## Use literal methods

A method can be used with either `some_library::some_module::some_method()` or `{some_literal}.some_method()`. For example:

```rust
use std::str;

str::replace("Hello World!", "!", "?");
```

Is the same as:

```rust
"Hello World!".replace("!", "?");
```

## Add conditional logic based on a set of values

```rust
match some_expression {
    some_value => // code to run,
    another_value => // code to run,
    _ => // code to run,
}
```

## Add conditional logic based on an either/or situation

```rust
if some_expression {
    ...
} else {
    ...
}
```

## Get the size of a collection

```rust
some_collection.len()
```

## Get whether a collection includes a value

```javascript
some_collection.contains(some_value);
```

## Access collection elements by index

```rust
some_collection[some_index]
```

## Get a portion of a collection by index (slice)

```rust
&some_collection[some_start_index..some_end_index]
```

## Transform a collection into an iterator

```rust
some_collection.iter()
```

## Get the size of an iterator

```rust
some_iterator.count()
```

## Do something for each item in an iterator

```rust
for some_item in some_iterator {
   // use some_item
}
```

## Apply a function on every item in an iterator

```rust
some_iterator.map(|some_item| // return updated value)
```

## Get items in an iterator that meet a condition

```rust
some_iterator.filter(|some_item| // return bool for the condition)
```

## Transform an iterator into a collection

```rust
some_iterator.collect()
```

## Compare values for equality

```rust
some_value == another_value
```

## Compare values for inequality

```rust
some_value != another_value
```

## Loop while a condition is met

```rust
while some_condition {
    ...
}
```

## Loop without data to iterate on

```rust
loop {
    // Some condition to stop looping
    break [some_return_value];

    // Some condition to jump to the next iteration
    continue;
}
```

## Use asynchronous functions

Calling `async` functions return a "future". The result of a future can be used with the `.await` operator.

```rust
async fn some_function() -> some_type {
    ...
}

async fn another_function() {
    let some_future = some_function();
    let some_result: some_type = some_future.await;
}
```

## Reuse variable name when converting the type of the value (shadowing)

```rust
let some_value = "42";
let some_value: u32 = some_value.trim().parse();
```

## Interpolate values in a string

```rust
format!("Some output with {} and {}", some_value, another_value, ...)
```

## Replace patterns in a string

```rust
some_string.replace("find", "replace")
```

## Get the individual characters in a string

```rust
some_string.chars()
```

## Convert a string to a number

```rust
some_string.trim().parse()
```

## Generate numbers in a sequence

```rust
(some_number..another_number)
```

## Add numbers

```rust
+
```

## Subtract numbers

```rust
-
```

## Multiply numbers

```rust
*
```

## Divide numbers

```rust
/
```

## Find the remainder after dividing numbers (modulus)

```rust
%
```

## Ignore unused values

```rust
_
```
