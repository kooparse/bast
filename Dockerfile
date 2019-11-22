# Select build image
FROM rust:latest

# Create a new empty shell project
RUN USER=root cargo new --bin bast
WORKDIR /bast

# Copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./migrations ./migrations
COPY ./static     ./static


# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/bast*
RUN cargo build --release

# Set the startup command to run your binary
CMD ["./target/release/bast"]
