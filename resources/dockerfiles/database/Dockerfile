FROM alpine:3.22 AS builder

COPY f1db.sql.gz /tmp/f1db.sql.gz
RUN gunzip /tmp/f1db.sql.gz

FROM mysql:latest

LABEL org.opencontainers.image.source=https://github.com/race-tech/f1-api
LABEL maintainer="Thibault C. <thibault.chene23@gmail.com>"

COPY --from=builder /tmp/f1db.sql /docker-entrypoint-initdb.d/f1db.sql
