FROM rust:1.78 as builder

WORKDIR /build
COPY . .
RUN cargo build -r

FROM fedora:latest

COPY --from=builder /build/target/release/basepack_repo_server /usr/local/bin/basepack-repo-server

ENTRYPOINT ["/usr/local/bin/basepack-repo-server"]