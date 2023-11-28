# axum-hotwire

> Rust implementation of the [Turbo Rails Tutorial](https://www.hotrails.dev/turbo-rails).

### Motivation and caveats

The primary motivation was to learn more about how [Hotwire Turbo](https://turbo.hotwired.dev/). Since
no particular backend is required I wanted to see what it would take to integrate with Rust. A secondary
motivation was a chance to investigate the following custom stack:

* [Axum](https://github.com/tokio-rs/axum)
* [Rusqlite](https://github.com/rusqlite/rusqlite)
* [markup.rs](https://github.com/utkarshkukreti/markup.rs)
* Rust / NPM web builds/tooling
* [Tailwind](https://tailwindcss.com/)
* Other important crates

Due to the motivations above and a lack of time there were some non-goals that resulted in some features
of the tutorial being left out:

* Broadcasting with WebSockets (Chapter 5)
* Security (Chapter 6)

Additionally, there were some other features and integral parts of Rails that haven't been replicated yet:

* The look and feel deviates from [demo](https://www.hotrails.dev/quotes) because the author has made some UI enhancements
* Viewports less than tablet sizing
* "to_sentence" on ValidationErrors struct for flash message
* Only add border color to fields with errors
* Labels for input fields
* Probably a few others

## Getting Started

### Without Docker

#### Prerequisites

* Rust version 1.67.1 or greater installed
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
* Build with `docker build -t axum-hotwire .`
* Run with `docker run -itd -e "DATABASE_URL=sqlite:///data/demo.db" -p 8080:8080 -v db-data:/data axum-hotwire`

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
* `fly secrets set DATABASE_URL=/data/demo.db`
* `docker build -t registry.fly.io/<GLOBALLY-UNIQUE-APP-NAME>:<VERSION-NUMBER> --target deploy .`
* `fly deploy --image registry.fly.io/<GLOBALLY-UNIQUE-APP-NAME>:<VERSION-NUMBER>`

## Automated deployment of new versions with GitHub [action](.github/workflows/deploy.yml)
* [Set up](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions) your `FLY_API_TOKEN` secret in your repository
* Tag release with a tag name starting with 'v'
  * Example: `git tag -a v2 -m "My new release!" && git push --tags`

## Manual deployment from local image
* `docker build -t registry.fly.io/<GLOBALLY-UNIQUE-APP-NAME>:<VERSION-NUMBER> --target deploy .`
* `fly auth docker`
* `docker push registry.fly.io/<GLOBALLY-UNIQUE-APP-NAME>:<VERSION-NUMBER>`
* `fly deploy --image registry.fly.io/<GLOBALLY-UNIQUE-APP-NAME>:<VERSION-NUMBER>`

