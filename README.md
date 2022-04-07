# Bast
![CI](https://github.com/kooparse/bast/workflows/CI/badge.svg)
<br/>
<br/>

# ⚠️ This project is not maintained anymore, so careful there.
<br/>
<br/>

Bast is an open sourced web analytics, giving simple informations about your site traffics. 

I did this project for myself. It is currently used by a couple of friends so it might be useful for others (if you’re a small business, an indie, a blogger…). This project is free and you can do whatever you want with it.

The backend is made in Rust using [actix-web](https://github.com/actix/actix-web) web framework and [diesel](https://github.com/diesel-rs/diesel) as PostgreSQL ORM. The frontend is (mostly) static and made in React with [Next](https://github.com/zeit/next.js). I used [chakra-ui](https://chakra-ui.com/) for the interface (but the graph is custom).

## Interface

<img src="https://i.imgur.com/DsE3xcM.png" alt="preview" />
<br/>

## Tracking
We track all metrics over time, days as our atomic scale.

**Pageviews** are anonymous or identified visitors, so 3 visitors equals 3 pageviews. On each pageview we store the **referrer**, the current **url**, etc... 

**Users** is the number of unique visitors to your website. We don’t rely on cookies to identify any particular user. We construct an identifier from his IP address and his user-agent (hashed and stored in the database), It’s really inspired by what [Ackee](https://github.com/electerious/Ackee) does. So if a user visits two pages, you will see 2 pageviews but only 1 user. 

A new **session** starts after 30 minutes of inactivity. So if a user opens your websites 3 times under 30 minutes, it counts as 1 session. If it's 1 time under 30 minutes and 1 time after; it counts as 2 sessions by the same user.

The **average time** is stored only if it’s between 5 secondes and 30 minutes on each sessions. 

We use the [woothee](https://github.com/woothee/woothee-rust) to parse and get information from user-agents; we get the **browser name**, **operating system** and other useful data.

And that’s it! Also, I would love to have the geolocation but not for now.

<br/>

## Contributing to the project

Don’t be shy about shooting any questions you may have. If you are a beginner/junior, don’t hesitate, I will always encourage you. It’s a safe place here. Also, I would be very happy to receive any kind of pull requests, you will have (at least) some feedback/guidance rapidly.

Behind screens, there are human beings, living any sort of story. So be always kind and respectful, because we all sheer to learn new things. See [contributing](https://github.com/kooparse/bast/blob/master/CONTRIBUTING.md) file for more exhaustive details.

<br/>

## Deploy Bast with Heroku
Easiest way to deploy the project!<br/>
<br/>
[![Deploy](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy)
<br/>
<br/>

## Install on your machine

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


## Running Bast with Docker on your machine

Once docker is installed on your machine, you just need to do: 
- `docker-compose up`
- Live at `http://localhost/3333`

Also you could customize all environment variables from `docker-compose.yml`

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

I read the [Fathom](https://github.com/usefathom/fathom) codebase, it’s a pretty cool project made in Golang. If you don’t want to spend time on hosting your own web analytics and you would like to pay for a service, you should definitely check their [plans](https://usefathom.com/pricing).

To identify users without using cookies, I was inspired by [Ackee](https://github.com/electerious/Ackee). So don't hesitate to check this cool project; it's fully made in javascript. 
