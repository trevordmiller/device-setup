# Asynchronous functions in Rust

Calling `async` functions return a "future". The result of a future can be used with the `.await` operator.

```rust
async fn some_function() {
  ...
}

async fn another_function() {
    let some_future = some_function();
    let some_result = some_future.await;
}
```
