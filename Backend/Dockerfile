# Rust version 
FROM rust:1.69.0 as build

# create a new empty shell project
# RUN USER=root cargo new --bin password_manager
WORKDIR /password_manager_webapp

# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y

# copy the files in local machine from the docker image
COPY . .

# set SQLX_OFFLINE var to true
ENV SQLX_OFFLINE true

# Build the binary in --release mode
RUN cargo build --release
RUN rm src/*.rs

# Now that the dependency is built, copy your source code
COPY ./src ./src

#  build for release
RUN rm ./target/release/deps/password_manager_webapp*
RUN cargo build --release

# final base
FROM rust:1-slim-bullseye

# copy the build artifact form the build stage
COPY --from=build password_manager_webapp/target/release/password_manager_webapp ./

# run the binary
USER root
CMD ["./password_manager_webapp"]
