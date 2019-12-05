# Example

Example using Rust Actix-web with juniper and diesel, connecting to postgreSQL (r2d2 - pooled connection)

## Running the example

If you haven't already, install diesel_cli.

```bash
cargo install diesel_cli --no-default-features --features postgres
```

Ensure that you have a postgreSQL server running and from the project root directory run.

```bash
echo "DATABASE_URL=[databaseUrl]" > .env
diesel migration run
cargo run
```

**_N.b. - Replace `[databaseUrl]` with your database url_**

### Open Graphql client

[http://localhost:8080/graphiql](http://localhost:8080/graphiql)

### Create a User

```graphql
mutation User($newUser: NewUser!) {
  createUser(newUser: $newUser) {
    id
    name
  }
}
```

**_query variables_**

```json
{
  "newUser": {
    "id": "123",
    "name": "Bob"
  }
}
```

### Get a User

```graphql
query User($id: String!) {
  user(id: $id) {
    id
    name
  }
}
```

**_query variables_**

```json
{
  "id": "123"
}
```
