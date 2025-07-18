#!/bin/bash

# Load environment variables
source .env

echo "Running database migrations..."
cd migration && cargo run -- up

echo "Migrations completed successfully!"