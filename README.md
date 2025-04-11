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

If this project is the first rust project, you have to install cargo cli to your machine.  

```
// install cargo
cargo install --force cargo-make
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
