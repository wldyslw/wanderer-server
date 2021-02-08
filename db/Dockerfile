FROM postgres:12-alpine
WORKDIR /db
COPY init.sql .
COPY init.sh /docker-entrypoint-initdb.d/
