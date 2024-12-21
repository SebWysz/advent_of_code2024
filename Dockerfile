FROM rust:1.83.0-slim

WORKDIR /src

# Install cargo-watch for auto-recompilation
RUN cargo install cargo-watch

# Mount code as a volume
VOLUME /src

# Use cargo watch to automatically recompile on changes
CMD ["cargo", "watch", "--poll", "-x", "run"]

