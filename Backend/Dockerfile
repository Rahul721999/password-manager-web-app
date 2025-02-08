# ------------------------- Prepare chef recipe ------------------------- #
FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /password_manager
RUN apt update && apt install -y lld clang

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ------------------------- Builder stage ------------------------- #
FROM chef AS builder
COPY --from=planner /password_manager/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

ARG SQLX_OFFLINE=true
# Build in release mode
RUN cargo build --release --bin password_manager_webapp

# ------------------------- Runtime stage ------------------------- #
FROM debian:bookworm-slim AS runtime
WORKDIR /password_manager
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends \
    openssl \
    libssl-dev \
    ca-certificates \
    # --------------------- Clean up --------------------- #
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /password_manager/target/release/password_manager_webapp ./

# Copy the migration folder
COPY migrations migrations
# Copy configurations 
COPY configuration configuration

COPY sqlx-data.json .

# ------------------------ Run migrations & start service ------------------------ #
USER root
CMD ["./password_manager_webapp"]
