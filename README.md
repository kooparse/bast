## Bast

![CI](https://github.com/kooparse/bast/workflows/CI/badge.svg)
<br/>
<br/>
Bast is an open sourced web analytics, giving simple informations about your site traffics.

I did this project for myself. It is tested and currently used by a couple of friends so it might be useful for others (if you’re a small business, an indie, a blogger…).

The backend is made in Rust using [actix-web](https://github.com/actix/actix-web) web framework and [diesel](https://github.com/diesel-rs/diesel) as PostgreSQL ORM. The frontend is (mostly) static and made in React with [Next](https://github.com/zeit/next.js).

### UI

<img src="https://i.imgur.com/uA3b099.png" alt="preview" />

## Install

### Unix and OS/X

### For the api.

- Fork or download this repository.
- `cd` to the project's location.
- `cargo install diesel_cli --no-default-features --features "postgres"`.
- `diesel setup` (You'll need postgres up and running).

### For the front.

- `cd` to website folder.
- `npm install`

<br/>

## Running the api in development

### For the api.

First create a new `.env` file at the root.
You should use `.env.sample` to get all required values.

If you want to watch the project while developping it, install `cargo-watch` first.

- `cargo install cargo-watch`
- run `cargo watch -i "website/**/*" -x run`
- Live at `http://localhost/3333`.

Otherwise simply run `cargo run`.

### For the front (website).

- `cd` to website folder.
- `npm run dev`
- Live at `http://localhost/3000`.
