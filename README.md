# aquarius

This is a simple project using poem-openapi and create some structure for organize the project

### Structure

- src
  - api
    - user
      - controller.rs
      - model.rs
      - utils.rs
      - service.rs If you need to add some logic to the controller, you can add it here
      - repository.rs If you need to create a repository to access the database, you can add it here

## How to run

```rust
cargo run
```

## Documentation

With Poem OpenAPI, you can easily generate OpenAPI documents for your Poem applications. Poem OpenAPI is based on the [OpenAPI 3.0](https://crates.io/crates/poem-openapi/3.0.5) specification.

## Dependencies

```rust
poem = "1.3.58"
poem-openapi =  { version = "3.0.5" , features = ["swagger-ui", "email"] }
slab = "0.4.9"
tokio = { version = "1.32.0", features = ["macros", "rt-multi-thread"] }
tracing-subscriber = "0.3.17"
```
