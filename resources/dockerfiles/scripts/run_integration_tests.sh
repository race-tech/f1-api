#!/bin/sh

# Check if docker is running
if ! docker info > /dev/null 2>&1; then
  echo "Docker is not running"
  exit 1
fi

# Create all env variables
export DB_IP_OR_HOSTNAME=mysql
export DB_PORT=3306
export DB_NAME=f1db
export DB_USER=user
export DB_PASSWORD=password
export REDIS_IP_OR_HOSTNAME=dragonfly
export REDIS_PORT=6379

# Create a network
docker network create purple-sector

# Start the database and cache
docker run -e MYSQL_DATABASE=${DB_NAME} -e MYSQL_USER=${DB_USER} -e MYSQL_PASSWORD=${DB_PASSWORD} --network purple-sector -h ${DB_IP_OR_HOSTNAME} -d thibaultcne/purple-sector:db-test --default-authentication-plugin=mysql_native_password
docker run -d --network purple-sector -h ${REDIS_IP_OR_HOSTNAME} docker.dragonflydb.io/dragonflydb/dragonfly:v1.16.1

# Run the tests
docker run -e DB_IP_OR_HOSTNAME -e DB_PORT -e DB_NAME -e DB_USER -e DB_PASSWORD -e REDIS_IP_OR_HOSTNAME -e REDIS_PORT --rm --network purple-sector purple-sector cargo test $@
