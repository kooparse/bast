# -----------------
# Front build stage
# -----------------
FROM node:12 as website

COPY ./website ./website
WORKDIR /website
RUN npm ci --production && npm run export

# -----------------
# Cargo build stage
# -----------------
FROM rust:1.42 as cargo-build
RUN USER=root cargo new --bin bast
WORKDIR /bast

COPY Cargo.lock .
COPY Cargo.toml .
COPY migrations ./migrations
COPY static     ./static
RUN mkdir -p ./static/front
RUN mkdir .cargo
RUN cargo vendor > .cargo/config

COPY ./server server
RUN rm -rf src/
RUN cargo build --release
RUN cargo install --path . --verbose

# -----------------
# Final stage
# -----------------
COPY --from=website /website/out /bast/static/front/

CMD ["/usr/local/cargo/bin/bast"]

EXPOSE 3333
