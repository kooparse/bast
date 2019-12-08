# -----------------
# Front build stage
# -----------------
FROM node:12.13.1-alpine as website

COPY ./website ./website
WORKDIR /website
RUN npm install && npm run export

# -----------------
# Cargo build stage
# -----------------
FROM rust:1.39 as cargo-build
RUN USER=root cargo new --bin bast
WORKDIR /bast

COPY Cargo.lock .
COPY Cargo.toml .
COPY migrations ./migrations
COPY static     ./static
RUN mkdir -p ./static/front
RUN mkdir .cargo
RUN cargo vendor > .cargo/config

COPY ./src src
RUN cargo build --release
RUN cargo install --path . --verbose

# -----------------
# Final stage
# -----------------
COPY --from=website /website/out /bast/static/front/

CMD ["/usr/local/cargo/bin/bast"]

EXPOSE 3333
