# Rust Sample API (Axum + Diesel)

A simple sample Rust REST API for demo purposes built with **Rust**, **Axum**, and **Diesel**, backed by a PostgreSQL database.

## Prerequisites

- **Rust**: [rustup.rs](https://rustup.rs/)
- **Docker & Docker Compose**: [docker.com](https://www.docker.com/)

## 1. Database Setup

The project includes a Docker Compose file to manage the database.

**Start the database container:**
```bash
docker-compose up -d
```

**Restore the Sample Database:**
This project uses the `dvdrental` sample database. The backup is located at `database/dvdrental.tar`.

**Option A: Using Docker Compose (Recommended)**
If your `docker-compose.yml` mounts the backup file to `/docker-entrypoint-initdb.d/`, simply restart the container:
```bash
docker-compose down -v
docker-compose up -d
```

**Option B: Manual Restore**
Run these commands to manually load the data:
```bash
docker exec -it dvdrental_db psql -U postgres -c "CREATE DATABASE dvdrental;"
docker exec -i dvdrental_db pg_restore -U postgres -d dvdrental < database/dvdrental.tar
```

## 2. Configuration

Create a `.env` file in the project root with the following content:

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/dvdrental
```

## 3. Run the Server

```bash
cargo run
```

The server will start on **port 3000**.

## 4. Testing

Once the server is running, you can test the following endpoints:

**GET /**
Returns a simple greeting.
```bash
curl http://localhost:3000/
```

**GET /customers**
Fetches the first 1000 customers from the database as JSON.
```bash
curl http://localhost:3000/customers
```
