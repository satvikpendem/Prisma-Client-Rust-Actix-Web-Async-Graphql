# Prisma-Client-Rust, Actix-Web, Async-Graphql

Example Actix Web server with Prisma (ORM) and GraphQL

The code is organized around a feature folder architecture, with GraphQL
operations such as queries, mutations, and subscriptions, inside of each folder.
Put whatever other logic is needed inside as well.

## Quickstart

- Create a database. We will use Postgres as an example, with a username and
  password of `postgres` and a database name of `prisma_rust_client_example`.

```bash
sudo -u postgres psql -c 'create database prisma_rust_client_example;'
```

- Add the database URL to `prisma/schema.prisma` (referenced as simply
  `schema.prisma` hereafter).

```prisma
...
datasource database {
  provider = "postgresql" // If you decide to use a different database, change the `provider` here
  url      = "postgresql://postgres:postgres@localhost:5432/prisma_rust_client_example"
}
...
```

- Run the following commands to initialize Prisma and set up the database:
  - `cargo prisma generate`: This will create the `src/prisma.rs` file which is
    code generated from reading the `schema.prisma` file
  - `cargo prisma migrate reset`: This will seed the database and create the
    necessary tables
  - (Optional) `npx prisma studio`: This brings up Prisma Studio, a GUI where
    you can add and remove rows in the database without needing to do it in the
    code itself. This _requires_ having NodeJS installed on your machine,
    otherwise `npx` will fail.

- Run `cargo run` to start the server.

- Navigate to `http://localhost:8000` to see GraphiQL where you can test out
  GraphQL operations.
