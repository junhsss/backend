FROM registry.gitlab.com/rust_musl_docker/image:stable-latest AS builder
WORKDIR /usr/src
RUN apt-get update && apt-get install ca-certificates -y
RUN rustup target add x86_64-unknown-linux-musl
RUN USER=root cargo new backend
WORKDIR /usr/src/backend
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN rm src/*.rs
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/backend*
RUN rm ./target/x86_64-unknown-linux-musl/release/backend*
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
ARG APP=/usr/src/app
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /usr/src/backend/target/x86_64-unknown-linux-musl/release/backend ${APP}/run
EXPOSE 8000
WORKDIR ${APP}
ENV RUST_LOG=INFO
CMD ["./run"]