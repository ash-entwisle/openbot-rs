FROM rust:alpine AS builder


ENV NAME=rustybox

WORKDIR /usr/src/app

COPY . .

RUN cargo install --path .
RUN cargo build --release

RUN cp ./target/release/${NAME} /usr/local/bin/app

FROM busybox:musl

WORKDIR /usr/local/bin

COPY --from=builder /usr/local/bin/app .

CMD ["./app"]
