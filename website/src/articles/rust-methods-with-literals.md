# Methods with literals in Rust

A method can be used with either `some_library::some_module::some_method()` or `{some_literal}.some_method()`. For example:

```rust
use std::str;

str::replace("Hello World!", "!", "?");
```

Is the same as:

```rust
"Hello World!".replace("!", "?");
```
