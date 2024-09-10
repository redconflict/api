# Redconflict Web API

## Folder structure

* `/src/` : Source folder.
  - `/main.rs` : Rust entry point (main).
* `/target` : Cargo build output cache.
* `/rustfmt.toml` : Rust code fomatter configuration.
* `/Cargo.toml` : Cargo package (iex: like package.json).
* `/Cargo.lock` : PM lock file (dependencies).
* `/.cargo`: Personnal configuration for app (iex: env, etc...) // SHOULD NEVER BE PUSHED ON GIT //

## How to run ?

Add `.cargo/config.toml` :

```toml
[env]
APP_ADDR = "0.0.0.0:8000"
DB_DSN = "postgres://root:root@localhost:5432/redconflict"
```

Then run :

`$ cargo run`

## Where binary are located ?

`target/<config>/<target_name>`

# api
