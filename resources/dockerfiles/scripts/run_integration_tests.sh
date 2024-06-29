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

# Create a network
docker network create f1-api

# Start the database and cache
docker run -e MYSQL_DATABASE=${DB_NAME} -e MYSQL_USER=${DB_USER} -e MYSQL_PASSWORD=${DB_PASSWORD} --network f1-api -h ${DB_IP_OR_HOSTNAME} -d --rm thibaultcne/purple-sector:db-test --default-authentication-plugin=mysql_native_password

sleep 5

# Run the tests
docker run --rm --network f1-api f1-api cargo test $@
TEST_RES=$?

# Stop the database and cache
docker stop $(docker ps -q --filter ancestor=thibaultcne/purple-sector:db-test)

# Remove the network
docker network rm f1-api

exit $TEST_RES
