
# -------------------------------
# Stage 1: Build
# -------------------------------
FROM rust:1.94.1 AS builder

WORKDIR /usr/src/obs-presign

# Copy manifest files first to maximize Docker layer cache stability.
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the actual application binary (no placeholder main).
RUN cargo build --release

# -------------------------------
# Stage 2: Runtime
# -------------------------------
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/src/obs-presign/target/release/obs-presign /app/obs-presign

RUN chmod +x /app/obs-presign

EXPOSE 3000

CMD ["/app/obs-presign"]