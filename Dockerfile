# FROM rust:latest
# WORKDIR /opt/backend
# COPY migrations ./migrations
# COPY src ./src
# COPY scripts ./scripts
# COPY Cargo.toml .
# RUN cargo build --release
# RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

# CMD [ "sh", "./scripts/run.sh" ]

FROM rust:latest as build
COPY src ./src
COPY Cargo.toml .
RUN cargo build --release
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres


FROM debian:stable-slim
COPY migrations /usr/migrations
COPY --from=build /usr/local/cargo/bin/sqlx /bin/sqlx
COPY --from=build ./target/release/technical-case-rust /bin/technical-case-rust
COPY scripts /bin

# CMD [ "./bin/minus_games_server" ]
CMD [ "sh", "/bin/run.sh" ]