FROM alpine:3.14 AS builder

COPY f1db.sql.gz /tmp/f1db.sql.gz
RUN gunzip /tmp/f1db.sql.gz

FROM mysql:latest

COPY --from=builder /tmp/f1db.sql /docker-entrypoint-initdb.d/f1db.sql
