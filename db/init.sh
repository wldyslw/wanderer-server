#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 -f /db/init.sql -v pass="'$POSTGRES_PASSWORD'"
