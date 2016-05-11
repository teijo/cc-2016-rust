# Codecamp 2016 Rust bootstrap

This is a simple [Rust](https://www.rust-lang.org/) bootstrap project for
[Reaktor's](https://reaktor.com/) spring 2016 codecamp. The project has
barebone JSON serialization and deserialization, and two-way UDP
communication.


## Development environment

- Install Rust from https://www.rust-lang.org/downloads.html
- Install dependencies with `cargo update`


## Running

- Test `cargo test`
- Application `cargo run`
  - Bootstrap code will expect an UDP server at `localhost:2000` which
    responds with JSON `{"name":<string>,"id":<integer>}`. You can emulate
    this by running UDP `netcat`: `nc -l -u 2000`, and writing the expected
    JSON string (followed by return) after getting the initial message from
    the app.
