#!/bin/sh

# Check if docker is running
if ! docker info > /dev/null 2>&1; then
  echo "Docker is not running"
  exit 1
fi

# Create all env variables
export DB_IP_OR_HOSTNAME=mysql
export DB_NAME=f1db
export DB_USER=user
export DB_PASSWORD=password
export REDIS_IP_OR_HOSTNAME=dragonfly

# Create a network
docker network create purple-sector

# Start the database and cache
docker run -e MYSQL_DATABASE=${DB_NAME} -e MYSQL_USER=${DB_USER} -e MYSQL_PASSWORD=${DB_PASSWORD} --network purple-sector -h ${DB_IP_OR_HOSTNAME} -d thibaultcne/purple-sector:db-test --default-authentication-plugin=mysql_native_password
docker run -d --network purple-sector -h ${REDIS_IP_OR_HOSTNAME} docker.dragonflydb.io/dragonflydb/dragonfly:v1.16.1

# Run the tests
docker run --rm --network purple-sector purple-sector cargo test $@

# Stop the database and cache
docker stop $(docker ps -q --filter ancestor=thibaultcne/purple-sector:db-test)
docker stop $(docker ps -q --filter ancestor=docker.dragonflydb.io/dragonflydb/dragonfly:v1.16.1)

# Remove the network
docker network rm purple-sector
