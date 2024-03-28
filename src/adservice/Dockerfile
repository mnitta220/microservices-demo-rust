# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.77.0
ARG APP_NAME=adservice

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /src

RUN apk add --no-cache clang lld musl-dev git make protobuf-dev

COPY . .
RUN cargo build --release

RUN GRPC_HEALTH_PROBE_VERSION=v0.3.1 && \
    wget -qO/bin/grpc_health_probe https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/${GRPC_HEALTH_PROBE_VERSION}/grpc_health_probe-linux-amd64 && \
    chmod +x /bin/grpc_health_probe

FROM alpine:3.18 AS final

COPY --from=build /src/target/release/adservice /bin/adservice
COPY --from=build /bin/grpc_health_probe /bin/

EXPOSE 9555

CMD ["/bin/adservice"]
