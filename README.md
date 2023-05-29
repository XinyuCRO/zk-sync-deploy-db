# zk-sync-deploy-db

This repository contains the scripts to setup & deploy the zkSync database.


## Usage

Start the docker first

### Setup


#### ENV

Duplicate `.env.example` as `.env`, and change the fields

#### Start

start the db docker image

```sh
cargo run start
```

#### Setup Database

Wait a few seconds for db to wake up, this command will create the database and run the migrations

```sh
cargo run init
```

#### Check Setup

After setup the db, run this command to print all tables in the db, to check if success

```sh
cargo run check
```

#### Print connection url to the db

```sh
cargo run print
```

### Management


#### Re-init db

```sh
cargo run reinit
```

#### Stop the db

```sh
cargo run stop
```

#### Sync schema migrations from upstream

This will sync the schema changes from https://github.com/matter-labs/zksync-era main branch

```sh
cargo run sync
```

#### Apply schema changes

```sh
cargo run migrate
```
