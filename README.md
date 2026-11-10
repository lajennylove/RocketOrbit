# RocketOrbit

The documentation site for [Orbit](https://github.com/lajennylove/Orbit), the ORM for Rocket.rs apps. Built on Rocket and [Propel](https://github.com/lajennylove/Propel) views, with Tailwind and Vite for the assets, like the RocketIgnite docs site.

The documentation covers the `orbit-orm` crate: connecting, models, queries, transactions, errors, migrations, the blocking API, and the differences from Laravel.

## Dependencies

The site is a Rocket app with Propel views. It does not depend on the Orbit crate.

- Rocket: git revision `3a54d079`
- Propel: tag `v0.2.0` (`propel_rocket`, `propel_runtime`, `propel_macros`, and `propel_compiler` as a build dependency)

## Developing this site

Node.js 22.12 or newer, or 20.19 or newer, is needed for Vite and Tailwind.

```sh
npm install
npm run build  # once: Rocket will not start until public/build exists
npm run dev    # Vite, in one terminal
cargo run      # Rocket, in another
cargo build --release   # a release build compiles the views in build.rs
```

A release build listens on `127.0.0.1:8004`, from `Rocket.toml`.

## Deploying

`.github/workflows/deploy.yml` builds the release binary and the assets on Ubuntu 24.04 in GitHub Actions, uploads them to the server, and switches the live release. It needs the secrets and the `DEPLOY_HOST` variable listed in the workflow.

## License

MIT. Copyright (c) 2026 Jenny Martinez.
