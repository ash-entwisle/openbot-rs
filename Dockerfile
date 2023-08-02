FROM rust:alpine AS builder


ENV NAME=openbot

WORKDIR /usr/src/app

COPY . .

RUN apk add libc-dev
RUN cargo install --path .
RUN cargo build --release

RUN cp ./target/release/${NAME} /usr/local/bin/app

FROM busybox:musl

WORKDIR /usr/local/bin

COPY --from=builder /usr/local/bin/app .

CMD ["./app"]
