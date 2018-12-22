FROM rust:1.31-slim-stretch

RUN apt-get update && \
    apt-get install -y libncurses-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /rlr
COPY include /rlr/

RUN cargo build

CMD ["cargo", "run"]
