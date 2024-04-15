#!/bin/sh

# Check if docker is running
if ! docker info > /dev/null 2>&1; then
  echo "Docker is not running"
  exit 1
fi

# Create all env variables
export DB_IP_OR_HOSTNAME=127.0.0.1
export DB_PORT=3306
export DB_NAME=f1db
export DB_USER=username
export DB_PASSWORD=password
export REDIS_IP_OR_HOSTNAME=127.0.0.1
export REDIS_PORT=6379

# Start the database and cache
docker run -e MYSQL_DATABASE=${DB_NAME} -e MYSQL_USER={DB_USER} -e MYSQL_PASSWORD={DB_PASSWORD} -p 3306:3306 -d thibaultcne/purple-sector:db-test --default-authentication-plugin=mysql_native_password
docker run -d -p 6379:6379 docker.dragonflydb.io/dragonflydb/dragonfly:v1.16.1

# Run the tests
docker run -e DB_IP_OR_HOSTNAME -e DB_PORT -e DB_NAME -e DB_USER -e DB_PASSWORD -e REDIS_IP_OR_HOSTNAME -e REDIS_PORT --rm purple-sector cargo test $@
