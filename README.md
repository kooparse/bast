## Bast

![CI](https://github.com/kooparse/bast/workflows/CI/badge.svg)
<br/>
<br/>
Bast is an open sourced web analytics, giving simple informations about your site traffics. 

I did this project for myself. It is currently used by a couple of friends so it might be useful for others (if you’re a small business, an indie, a blogger…). This project is **free** and you can do whatever you want with it.

The backend is made in Rust using [actix-web](https://github.com/actix/actix-web) web framework and [diesel](https://github.com/diesel-rs/diesel) as PostgreSQL ORM. The frontend is (mostly) static and made in React with [Next](https://github.com/zeit/next.js). I used [chakra-ui](https://chakra-ui.com/) for the interface (but the graph is custom).

### UI

<img src="https://i.imgur.com/uA3b099.png" alt="preview" />
<br/>

## Contribution

Don’t be shy about shooting any questions you may have. If you are a beginner/junior, don’t hesitate, I will always encourage you. It’s a safe place here. Also, I would be very happy to receive any kind of pull requests, you will have (at least) some feedback/guidance rapidly.

Behind screens, there are human beings, living any sort of story. So be always kind and respectful, because we all sheer to learn new things.
<br/><br/>

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

<br/>

## Thanks

I read the [Fathom](https://github.com/usefathom/fathom) codebase, it’s a pretty cool project made in Golang, also their codebase is clean. If you don’t want to spend time on hosting your own web analytics, you should definitely check their [plans](https://usefathom.com/pricing).
