FROM rust:latest

ENV service_dir /home/rust/server

WORKDIR ${service_dir}
COPY ./ ${service_dir}/

RUN apt-get update && apt-get install -y cmake
RUN apt-get install protobuf-compiler -y
RUN GRPC_HEALTH_PROBE_VERSION=v0.3.1 && \
    wget -qO/bin/grpc_health_probe https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/${GRPC_HEALTH_PROBE_VERSION}/grpc_health_probe-linux-amd64 && \
    chmod +x /bin/grpc_health_probe

EXPOSE 3550

RUN cargo build --release --bin server
CMD ${service_dir}/target/release/server
