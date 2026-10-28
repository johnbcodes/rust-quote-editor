# rust-quote-editor

> Rust implementation of the quote editor from [Turbo Rails Tutorial](https://www.hotrails.dev/turbo-rails).

### Motivation and caveats

The main motivation is learning to develop web applications with Rust and JavaScript combined. It now includes
the following stack:

* [htmx](https://htmx.org/)
* [hyperscript](https://hyperscript.org/)
* [Rocket](https://rocket.rs/)
* [Diesel](https://diesel.rs/)
* [markup.rs](https://github.com/utkarshkukreti/markup.rs)
* [Vite+](https://viteplus.dev)
* [Tailwind](https://tailwindcss.com/)

In the past it included these technologies:

* [Hotwire Turbo](https://turbo.hotwired.dev/)
* [Axum](https://github.com/tokio-rs/axum)
* [Rusqlite](https://github.com/rusqlite/rusqlite)

Some features of the tutorial were intentionally left out and possibly will be worked on in the future:

* Broadcasting with WebSockets (Chapter 5)
* Security (Chapter 6)

Additionally, there were some other features that haven't been replicated yet:

* The look and feel deviates from the Turbo Rails Tutorial [demo](https://www.hotrails.dev/quotes) because the author has made some UI enhancements that are not in the tutorial
* Viewports less than tablet sizing
* Labels for input fields
* Delete confirmation
* ...probably a few others

## Getting Started

### Without Docker

#### Prerequisites

* Rust version 1.97.1 installed
* Vite+ installed (`vp`; see https://viteplus.dev/guide/)
* Node.js 24 (managed automatically by Vite+)

#### Install and build

* Install frontend dependencies `vp install`
* Build web with `vp build` (or `npm run build`)
* Install Rust dependencies `cargo install`
* Build with `cargo build`
* Run with `cargo run`

### With Docker

#### Prerequisite

* Docker and Docker Compose or compatible software installed.

#### Docker only

* Create volume with `docker volume create db-data`
* Build with `docker build -t rust-quote-editor .`
* Run with `docker run -itd -p 8080:8080 -v db-data:/data rust-quote-editor`

#### Docker Compose

* Build with `docker compose build`
* Run with `docker compose up` or `docker compose up -d` (build step not necessary once built)
