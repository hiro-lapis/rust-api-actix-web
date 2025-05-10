# rust-api-actix-web

### set up

Each task defined in Makefile.toml can be excuted by `cargo make xxx`  
Note that `cargo run` seems to work, but fails in reading env vars.  


```
// run [tasks.run-in-docker]
cargo make run-in-docker

// during development, if you want to restart local
cargo make compose-remove \
cargo make build \
cargo make initial-setup \
cargo make run
```
Optionally, conducive commands are executable.  

```
// start creating new crate
cargo new --lib <crate name>
```

Also, common commands are usable.  

```
docker container exec -it rust-api-app-1 bash
```

### set up on your laptop

1. cargo CLI  

If this project is the first rust project, you have to install cargo cli to your machine.  

```
cargo install --force cargo-make

// if version can be seen, instalation is succeeded.
cargo --version
```

2. [diesel CLI](https://diesel.rs/guides/getting-started)  

This project also uses diesel. In order to operate on CLI, please install diesel CLI.  

```
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh
```

### migration

```
diesel migration <run/revert/redo>
diesel migration list
```

If you want to get to know more about migration, `diesel migration help` is useful.  

When raw DDL SQL is running, diesel automatically generate or update `schema.rs`, which expresses the DDL in rust code and generated in the place defined by `diesel.toml`. This is used for ORM management.  
Make sure not modify `schema.rs` manually!  


### urls

- `http://127.0.0.1:8080` served as graphql api playground.  
