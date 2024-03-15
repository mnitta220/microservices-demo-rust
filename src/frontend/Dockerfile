FROM rust:1.76-alpine3.19 as builder
RUN apk update && apk add --no-cache gcc libc-dev make protobuf-dev
WORKDIR /src
COPY . .
RUN cargo build --release

FROM alpine:3.19
WORKDIR /src
COPY --from=builder /src/target/release/frontend /src/frontend

EXPOSE 8080
ENTRYPOINT [ "/src/frontend" ]
