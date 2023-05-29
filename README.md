# zk-sync-deploy-db

zk-sync-deploy-db is a repository that contains scripts to set up and deploy the zkSync database. This README provides instructions on how to use the scripts.

## Usage

Before using the scripts, start the Docker.

### Setup

#### Environment Variables

Duplicate the `.env.example` file as `.env` and make the necessary changes to the fields.

#### Starting the Database Docker Image 

To start the database Docker image, run the following command:

```sh
cargo run start
```

#### Setting up the Database

After the Docker image has started, create the database and run the migrations using the following command:

```sh
cargo run init
```

#### Checking the Setup

To check if the setup of the database was successful, run the following command to print all tables in the database:

```sh
cargo run check
```

#### Printing the Connection URL to the Database

To print the connection URL to the database, run the following command:

```sh
cargo run print
```

### Management

#### Re-initializing the Database

To re-initialize the database, run the following command:

```sh
cargo run reinit
```

#### Stopping the Database

To stop the database, run the following command:

```sh
cargo run stop
```

#### Syncing Schema Migrations from Upstream

To sync the schema changes from the main branch of https://github.com/matter-labs/zksync-era, runthe following command:

```sh
cargo run sync
```

#### Applying Schema Changes

To apply schema changes, run the following command:

```sh
cargo run migrate
```

## License

zk-sync-deploy-db is licensed under [MIT License](https://github.com/matter-labs/zk-sync-deploy-db/blob/main/LICENSE).