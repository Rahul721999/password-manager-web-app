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
# Build out project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
RUN cargo install sqlx-cli
# Upto this point, if our dependency tree stays the same
# all layers should be cached.
COPY . .
# Set env var sql_offline to TRUE
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

# ------------------ Set App_Env to PRODUCTION ------------------ #
ENV APP_ENVIRONMENT=production

# ------------------------ Run migrations & start service ------------------------ #
USER root
# CMD ["sh", "-c", "./sqlx migrate run && ./password_manager_webapp"]
CMD ["./password_manager_webapp"]
