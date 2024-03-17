FROM rust:1.76-alpine3.19 as builder
RUN apk update && apk add --no-cache gcc libc-dev make protobuf-dev
WORKDIR /src
COPY . .
RUN cargo build --release --bin server

FROM alpine:3.19
WORKDIR /src
COPY --from=builder /src/target/release/server /src/server
COPY --from=builder /src/products.json /src/products.json

RUN GRPC_HEALTH_PROBE_VERSION=v0.3.1 && \
    wget -qO/bin/grpc_health_probe https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/${GRPC_HEALTH_PROBE_VERSION}/grpc_health_probe-linux-amd64 && \
    chmod +x /bin/grpc_health_probe

EXPOSE 3550
ENTRYPOINT [ "/src/server" ]
