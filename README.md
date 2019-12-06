## Bast

A website analytics.

## Install

### Unix and OS/X

### For the api.

- Fork or download this repository.
- `cd` to the project's location.
- `cargo install diesel_cli --no-default-features --features "postgres"`
- `diesel setup`

### For the front.

- `cd` to website folder.
- `npm install`

## Running the api in development

### For the api.

First create a new `.env` file at the root.
You should use `.env.sample` to get all required values.

If you want to watch the project while developping it, install cargo-watch first.

- `cargo install cargo-watch`
- run `cargo watch -i "website/**/*" -x run`
- Live at `http://localhost/3333`.

Otherwise simply run `cargo run`.

### For the front (website).

- `cd` to website folder.
- `npm run dev`
- Live at `http://localhost/3000`.
