#!/usr/bin/env bash
#
# Run on your machine to initialize your database.

abort() {
    [ "$1x" != "x" ] && echo $1
    echo "Aborted."
    exit 1;
}

usage() {
    echo "This script will initialize the database and run all migrations."
    echo ""
    echo "Make sure sea-orm-cli and psql tools are available."
    echo ""
    echo "Set SKIP_DOCKER to SKIP_DOCKER=1 in order to skip running a Docker container"
    echo "with PostgreSQL."
    echo ""
    echo "Optionally, following environment variables can be set: "
    echo 'DB_USER     -> default: POSTGRES_USER     or "postgres"'
    echo 'DB_PASSWORD -> default: POSTGRES_PASSWORD or "password"'
    echo 'DB_NAME     -> default: POSTGRES_NAME     or "newsletter"'
    echo 'DB_PORT     -> default: POSTGRES_PORT     or "5432"'
    echo 'DB_HOST     -> default: POSTGRES_HOST     or "localhost"'
    echo ""
    exit 0;
}

if [ "$1" == "-h" ] || [ "$1" == "--help" ]; then
    usage
fi

[ ! -x "$(command -v psql)" ] && abort "Error: 'psql' command not found"
[ ! -x "$(command -v sea-orm-cli)" ] && abort "Error: 'sea-orm-cli' command not found -> check README.md"

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD=${POSTGRES_PASSWORD:=password}
DB_NAME=${POSTGRES_NAME:=newsletter}
DB_PORT=${POSTGRES_PORT:=5432}
DB_HOST=${POSTGRES_HOST:=localhost}

export PGPASSWORD="${DB_PASSWORD}"

# Allow to skip Docker if a dockerized Postgres database is already running
TEST=`psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c 'SELECT NOW();'`
if [ -z "${TEST}" ]; then
    if [ -z "${SKIP_DOCKER}" ]; then
        docker run \
            -e POSTGRES_USER=${DB_USER} \
            -e POSTGRES_PASSWORD=${DB_PASSWORD} \
            -e POSTGRES_DB=${DB_NAME} \
            -p "${DB_PORT}":5432 \
            -d postgres:14 \
            postgres -N 1000
    fi
fi

until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    echo "Postgres is still unavailable - sleeping ... press CTRL-C to stop this script"
    sleep 1
done

echo "Postgres is running on ${DB_HOST}:${DB_PORT}."

export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"

TEST=`psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -tAc "SELECT 1 FROM pg_database WHERE datname='${DB_NAME}';"`
if [ -z "${TEST}" ]; then
    echo "Will create database first."
    psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -tAc "CREATE DATABASE '${DB_NAME}';" || abort "Could not create database ${DB_NAME}."
fi

echo "Running migrations now."
sea-orm-cli migrate up || abort "Could not run migrations."

echo "Done"
