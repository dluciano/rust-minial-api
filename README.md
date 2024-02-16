# Web Service Tutorial in Rust

## Overview

This project serves as a comprehensive tutorial for building a web service using the Rust programming language. It covers various aspects, from setting up a PostgreSQL database to handling JSON responses.

## Technologies Used

- **Rust**: The core programming language used for this project.
- **Tokio**: An asynchronous runtime for Rust.
- **SQLx**: A Rust library for connecting to PostgreSQL databases.
- **Dotenv**: A Rust crate for handling `.env` files.
- **Serde**: A Rust crate for serializing and deserializing data structures.
- **Actix-Web**: A Rust framework for building web services.

## Documentation

To generate documentation for this project:

```bash
cargo doc --open
```

Alternatively, you can find documentation for each crate on [docs.rs](https://docs.rs/).

## Getting Started

### Prerequisites

- Ensure you have Docker Compose installed. If not, you can download it from [Docker Desktop](https://www.docker.com/products/docker-desktop).

### Setup

1. **Environment Variables**: Copy the sample environment file and configure the variables.

   ```bash
   cp env.example .env
   ```

2. **Docker Compose**: Start the Docker containers.

   ```bash
   docker compose up -d
   ```

3. **Compile Queries**:

   ```bash
   cargo sqlx prepare
   ```

4. **Upload Postman Collection**: Open up Postman locally via the desktop client. Then upload the file entitled `Web Service Tutorial.postman_collection.json`. You should now have access to the Postman collection.

5. **Test Routes**: Open your browser or use a tool like Postman to hit the following route:

   ```
   http://127.0.0.1:8080/blog
   ```

### SQL Schema

The `init.sql` file contains the SQL statements that define the database schema. Feel free to explore it to understand the database structure.

## Additional Resources

- [Crates.io Package](https://crates.io/crates/webservice_tutorial)
