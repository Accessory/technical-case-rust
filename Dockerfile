FROM rust:latest
WORKDIR /opt/backend
COPY migrations ./migrations
COPY src ./src
COPY scripts ./scripts
COPY Cargo.toml .
RUN cargo build --release
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

CMD [ "sh", "./scripts/run.sh" ]