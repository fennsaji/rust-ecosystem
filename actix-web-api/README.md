# Actix-Web API with PostgreSQL

A clean architecture REST API built with Actix-web and SeaORM.

## Features

- Clean Architecture with clear separation of concerns
- PostgreSQL database with SeaORM
- User CRUD operations
- Dependency injection
- Comprehensive error handling
- Database migrations
- Async/await throughout

## Local Setup

### Prerequisites

1. **Install Rust** (if not already installed)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Install PostgreSQL**
   
   **macOS (using Homebrew):**
   ```bash
   brew install postgresql
   brew services start postgresql
   ```
   
   **Ubuntu/Debian:**
   ```bash
   sudo apt update
   sudo apt install postgresql postgresql-contrib
   sudo systemctl start postgresql
   sudo systemctl enable postgresql
   ```
   
   **Windows:**
   Download and install from [PostgreSQL official website](https://www.postgresql.org/download/windows/)

3. **Install SeaORM CLI** (optional, for advanced database operations)
   ```bash
   cargo install sea-orm-cli
   ```

### Database Setup

1. **Create PostgreSQL user and database**
   ```bash
   # Connect to PostgreSQL as postgres user
   sudo -u postgres psql
   
   # Create database and user
   CREATE DATABASE actix;
   CREATE USER your_username WITH ENCRYPTED PASSWORD 'your_password';
   GRANT ALL PRIVILEGES ON DATABASE actix TO your_username;
   \q
   ```

2. **Configure environment variables**
   
   Create a `.env` file in the project root:
   ```bash
   cp .env.example .env
   ```
   
   Update `.env` with your database credentials:
   ```env
   DATABASE_URL=postgres://your_username:your_password@localhost:5432/actix
   ```

3. **Verify database connection**
   ```bash
   # Test connection (optional)
   psql $DATABASE_URL -c "SELECT version();"
   ```

## Running Database Migrations

### Method 1: Using the provided script (Recommended)

```bash
# Make script executable (if not already)
chmod +x run_migrations.sh

# Run migrations
./run_migrations.sh
```

### Method 2: Using SeaORM CLI (if installed)

```bash
# From project root
sea-orm-cli migrate up

# Check status
sea-orm-cli migrate status
```

## Running the Application

### Development Mode

```bash
# Install dependencies and run
cargo run

# Run with logging
RUST_LOG=debug cargo run

# Run on different port
PORT=3000 cargo run
```

### Production Mode

```bash
# Build optimized binary
cargo build --release

# Run the optimized binary
./target/release/actix-web-api
```

The server will start on `http://localhost:8080` by default.

## API Endpoints

- `GET /health` - Health check
- `POST /users` - Create user
- `GET /users` - List all users
- `GET /users/{id}` - Get user by ID
- `PUT /users/{id}` - Update user
- `DELETE /users/{id}` - Delete user

## Example Usage

### Create a user
```bash
curl -X POST http://localhost:8080/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "email": "john@example.com"}'
```

### Get all users
```bash
curl http://localhost:8080/users
```

### Get user by ID
```bash
curl http://localhost:8080/users/{user_id}
```

### Update user
```bash
curl -X PUT http://localhost:8080/users/{user_id} \
  -H "Content-Type: application/json" \
  -d '{"name": "Jane Doe", "email": "jane@example.com"}'
```

### Delete user
```bash
curl -X DELETE http://localhost:8080/users/{user_id}
```