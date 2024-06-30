#!/bin/bash

echo "Building Docker image..."
# Build the Docker image from the directory of Dockerfile
docker build -t surrealdb-container /home/filipg/master/ZKAuth

echo "Running Docker container..."
# Run the Docker container
docker run -d -p 8000:8000 surrealdb-container

echo "SurrealDB container is up and running."