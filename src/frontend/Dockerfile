# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.77.0
ARG APP_NAME=frontend

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /src

RUN apk update && apk add --no-cache clang lld musl-dev git make protobuf-dev

COPY . .
RUN cargo build --release
COPY ./static /src/static

FROM alpine:3.18 AS final
WORKDIR /src

COPY --from=build /src/target/release/frontend /src/frontend
COPY --from=build /src/static /src/static

# ENV_PLATFORM: One of: local, gcp, aws, azure, onprem, alibaba
ENV ENV_PLATFORM=local

EXPOSE 8080

CMD ["/src/frontend"]
