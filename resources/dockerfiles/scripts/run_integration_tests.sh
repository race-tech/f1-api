#!/bin/sh

# Check if docker is running
if ! docker info > /dev/null 2>&1; then
  echo "Docker is not running"
  exit 1
fi

# Start the database and cache
docker run -e MYSQL_DATABASE -e MYSQL_USER -e MYSQL_PASSWORD -p 3306:3306 -d thibaultcne/purple-sector:db-test --default-authentication-plugin=mysql_native_password
docker run -d -p 6379:6379 docker.dragonflydb.io/dragonflydb/dragonfly:v1.16.1

# Run the tests
docker run --rm purple-sector cargo test $@
