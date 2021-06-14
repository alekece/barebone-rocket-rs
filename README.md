# simple-rocket-rs

A barebones [Rocket](https://rocket.rs) API with [JWT](https://jwt.io) authentication and 
database integration. 

## Requirement
- Rust version 1.52 or newer.
- Diesel CLI with `postgres` or `sqlite` features.
- A running [PostgreSQL](https://www.postgresql.org/) or [Sqlite](https://www.sqlite.org/index.html) backend.
  
## Setup environment

A `.env.example` at the root directory exposes environment both used by `diesel`and the project itself.  
Rename it to `.env` then set all the environment variables before running the following commands :

``` bash
source .env
```

## Build locally

Run the following command to fulfill the requirements :

``` bash
# Install Rust and cargo alongside rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install ORM and query builder
cargo install diesel_cli --no-default-features --features "postgres sqlite"

# Install the desired through docker or locally

# Run migrations
diesel setup 
diesel migration run
```

Then build the project :


``` bash
cargo build --release --no-default-features --features <postgres|mysql>
```

## Switching backend

Any time you want to perform a backend switch, you have to :
2. Install and run the desired database on your machine if is not already the case
3. Run migrations
4. Build the project with the respective feature flag.

## Usage

To print the project usage, an option `-h` is available.

## API routes

``` http

POST /authenticate
{
"username": "string",
"password": "string"
}

GET /users
Authorization: Bearer <token>

POST /users
Authorization: Bearer <token>
{
"username": "string",
"password": "string",
"email": "string",
"is_admin": boolean
}


DELETE /users/<username>
Authorization: Bearer <token>
```
