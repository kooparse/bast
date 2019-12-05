# Select build image
FROM rust:latest

RUN curl -sL https://deb.nodesource.com/setup_13.x | bash -
RUN apt-get install -y nodejs

# Create a new empty shell project
RUN USER=root cargo new --bin bast
WORKDIR /bast

COPY ./website ./website
WORKDIR /bast/website
RUN npm install
RUN npm run export

WORKDIR /bast

# Copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./migrations ./migrations
COPY ./static     ./static
COPY ./website/out ./static/front

RUN cargo build --release


# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/bast*
RUN cargo build --release

# Set the startup command to run your binary
CMD ["./target/release/bast"]
