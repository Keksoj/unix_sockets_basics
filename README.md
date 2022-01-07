# The basics of unix sockets

This repository serves as a reference for [this tutorial blogpost](emmanuelbosquet.com)

## How to run

[Install Rust and Cargo](https://www.rust-lang.org/tools/install),
and then do:

    rm mysocket
    cargo run --bin server

And in a separate terminal, run the client:

    cargo run --bin client

If all is well,

-   the hello message should display on the server side
-   the "I hear you" response should display on the client side

You can run the client as many times as you want, since the server runs in a loop.
