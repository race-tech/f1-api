FROM alpine:3.14 AS builder

RUN apk add --no-cache wget

RUN wget https://ergast.com/downloads/f1db.sql.gz -P /tmp
RUN gunzip /tmp/f1db.sql.gz

FROM mysql/mysql-server:latest

COPY --from=builder /tmp/f1db.sql /docker-entrypoint-initdb.d/f1db.sql
