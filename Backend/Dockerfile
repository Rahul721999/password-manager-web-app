# prepare chef reciepe
FROM lukemathwalker/cargo-chef:latest-rust-1.69.0 as chef
WORKDIR /password_manager
RUN apt update && apt install lld clang -y
FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage
FROM chef as builder
COPY --from=planner /password_manager/recipe.json recipe.json
# Build out project dependecies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# upto this point, if our dependecy tree stays the same
# all layers should be cached.
COPY  . .
# set env var sql_offline to TRUE
ENV SQLX_OFFLINE true
# build in relase mode
RUN cargo build --release --bin password_manager_webapp
# clean build environment
RUN rm src/*.rs

# Now that the dependency is built, copy your source code
COPY ./src ./src
#  build for release
RUN rm ./target/release/deps/password_manager_webapp*
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim AS runtime
WORKDIR /password_manager
RUN apt-get update -y \
&& apt-get install -y --no-install-recommends openssl ca-certificates \
# Clean up
&& apt-get autoremove -y \
&& apt-get clean -y \
&& rm -rf /var/lib/apt/lists/*

COPY --from=builder /password_manager/target/release/password_manager_webapp ./

COPY configuration configuration
COPY sqlx-data.json .
# Set App_Env to PRODUCTION
ENV APP_ENVIRONMENT production
# run the binary
USER root
CMD ["./password_manager_webapp"]

