FROM node:12.13.1-alpine as website

WORKDIR /website
COPY ./website/package.json /website/package.json
COPY ./website/package-lock.json /website/package-lock.json
RUN npm install
COPY ./website /website
RUN npm run export

FROM rust:1.39

# Create a new empty shell project
RUN USER=root cargo new --bin bast

WORKDIR /bast

# Copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./migrations ./migrations
COPY ./static     ./static
COPY --from=website /website/out /bast/website/static

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

EXPOSE 3333
