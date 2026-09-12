# rust-api-actix-web

Boiler plate of graphql api composed with actix-web.  

1. setup
2. commands
3. project information
3. dependencies' reference

## 1. set up

Each task defined in Makefile.toml can be excuted by `cargo make xxx`  
Note that `cargo run` seems to work, but fails in reading env vars.  

Basically, you can build develop server on your laptop.  

## FE

```
cd frontend
npm install
npm run dev
```

### GraphQL type generation

The frontend types and react-query hooks in `frontend/gen/graphql.ts` are generated from the
running backend via GraphQL introspection. **Start the backend first**, then run codegen.

```
# terminal 1
cd backend && cargo make run

# terminal 2
cd frontend && npm run codegen
```

Queries live in `frontend/app/lib/queries/*.graphql`. Re-run `npm run codegen` after changing
either those documents or the backend schema, and commit the regenerated `frontend/gen/graphql.ts`.
The schema URL defaults to `http://localhost:8080/` and can be overridden with `GRAPHQL_SCHEMA_URL`.

## BE
```
cargo make run

# `http://127.0.0.1:8080` served as graphql api playground.
```

Alternatively, run on docker.  

```
// run [tasks.run-in-docker]
cargo make run-in-docker

// during development, if you want to restart local
cargo make compose-remove \
cargo make build \
cargo make initial-setup \
```

That's all! Let's enjoy dev life!
If you are first to rust dev, let's move to next headline to install some cli.

### set up on your laptop

1. cargo CLI  

If this project is the first rust project, you have to install cargo cli to your machine.  

```
cargo install --force cargo-make

// if version can be seen, instalation is succeeded.
cargo --version
```

2. [diesel CLI](https://diesel.rs/guides/getting-started)  

This project also uses diesel. In order to manage migration, install diesel CLI.  

```
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh
```

## 2. commands

### migration

```
DATABASE_URL="postgresql://localhost:5432/app?user=app&password=passwd" diesel migration <run/revert/redo>
diesel migration list

// via makefile
cargo make migration-list
```

If you want to get to know more about migration, `diesel migration help` is useful.  

When raw DDL SQL is running, diesel automatically generate or update `schema.rs`, which expresses the DDL in rust code and generated in the place defined by `diesel.toml`. This is used for ORM management.  
Make sure not modify `schema.rs` manually!  

### create new crate

```
// start creating new crate
cargo new --lib <crate name>

### docker container

```
// dive in app container
docker container exec -it rust-api-app-1 bash

// if you change container setting eg.outer port, do this. since some env's are defined in Makefile.toml, you need restart via cargo-make
cargo make compose-down
cargo make compose-up

// if you want to query in db container, do this in db container
psql -U app -d app
```


## Tech stacks

- [Actix Web](https://actix.rs/)
- [Diesel/Diesel CLI](https://diesel.rs/)
- [Next.js]()
