# rlr - Rustlike Revisited

This shouldn't go anywhere. I've forgotten everything about rust, so I'm tinkering again.

The plan as of now is to run through [the roguelike tutorial revised](http://rogueliketutorials.com/tdl/1) until I get stuck.

## Running

`env_logger` accepts a log level from an env var, and outputs to stderr. Therefore, to run with logging, we can use:

    RUST_LOG=debug cargo run 2> rlr.log

To simulate a slow connection, you can use `pv`

    cargo run | pv -L 512 -q

### Docker

To build and run in a docker image:

    ./docker-run.sh

To quickly start hacking on the code in a docker image:

    ./docker-dev.sh

From there you can use `cargo build` and `cargo run`.
