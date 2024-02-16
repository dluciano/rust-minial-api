ARG ARCH=
FROM ${ARCH}rust:alpine3.19@sha256:ec93a9ad3065df593645171a3aa6c47b55578914d2c232860260dbd27bb0cbc0 as build

# The presence of a DATABASE_URL environment variable will take precedence over the presence of .sqlx, meaning SQLx will default to building against a database if it can. To make sure an accidentally-present DATABASE_URL environment variable or .env file does not result in cargo build (trying to) access the database, you can set the SQLX_OFFLINE environment variable to true.
ENV SQLX_OFFLINE=true
ENV MODE=prod

# Install requirements needed to build the rust project using Alpine
RUN apk update && apk add alpine-sdk=1.0-r1 && apk add openssl-dev=3.1.4-r5

# Build the rust project
WORKDIR /usr/src/
RUN cargo new webapi
WORKDIR /usr/src/webapi
COPY . .
RUN cargo build --release

# Prepare the production docker image layer
FROM ${ARCH}alpine:3.19@sha256:6457d53fb065d6f250e1504b9bc42d5b6c65941d57532c072d929dd0628977d0 as prod

RUN addgroup --gid 1000 -S app && adduser --uid 1000 -S app -G app

WORKDIR /var/www/html/webapi

COPY --from=build /usr/src/webapi/target/release/webapi .

RUN chown -R app:app /var/www/html/webapi

USER app
EXPOSE 80
ENV HOST=0.0.0.0
ENV PORT=80

ENTRYPOINT ["/var/www/html/webapi/webapi"]
