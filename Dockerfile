FROM --platform=$BUILDPLATFORM rust AS build

RUN apt-get update && apt-get install -y musl-tools musl-dev protobuf-compiler ca-certificates && update-ca-certificates

WORKDIR /app

RUN cargo init
COPY Cargo.* .

RUN cargo build --release

COPY src src
RUN touch src/main.rs && cargo build --release && mv target/release/RpBot_reborn .

FROM rust
WORKDIR /app
COPY --from=build /app/RpBot_reborn .
COPY .env .
COPY translations translations
USER 65534:65534
ENTRYPOINT ["./RpBot_reborn"]
CMD []
