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
* Custom Rust / NPM build integration
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

* Rust version 1.78.0 or greater installed
* NodeJS version 20 or greater installed

#### Install and build

* Install Node dependencies `npm install`
* Build web with `npm run build`
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

## Initial deployment to fly.io with `flyctl` (aliased to `fly`)
* Create account if necessary
* `fly auth login`
* `fly apps create <GLOBALLY-UNIQUE-APP-NAME>`
  * Update `app` property in `fly.toml` with <APP-NAME>
* Choose fly.io region
  * Update `primary_region` property in `fly.toml`
* `fly volumes create <VOLUME-NAME> -s 1 -r <REGION>`
  * Update `mounts.source` property in `fly.toml` with <VOLUME-NAME>
* `docker build -t registry.fly.io/<GLOBALLY-UNIQUE-APP-NAME>:<VERSION-NUMBER> --target deploy .`
* `fly deploy --image registry.fly.io/<GLOBALLY-UNIQUE-APP-NAME>:<VERSION-NUMBER>`

## Automated deployment of new versions with GitHub [action](.github/workflows/deploy.yml)
* [Set up](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions) your `FLY_API_TOKEN` [secret](https://fly.io/docs/reference/deploy-tokens/) in your repository
* Tag release with a tag name starting with 'v'
  * Example: `git tag -a v2 -m "My new release!" && git push --tags`

## Manual deployment from local image
* `docker build -t registry.fly.io/<GLOBALLY-UNIQUE-APP-NAME>:<VERSION-NUMBER> --target deploy .`
* `fly auth docker`
* `docker push registry.fly.io/<GLOBALLY-UNIQUE-APP-NAME>:<VERSION-NUMBER>`
* `fly deploy --image registry.fly.io/<GLOBALLY-UNIQUE-APP-NAME>:<VERSION-NUMBER>`
