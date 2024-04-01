# Rust での書き直し

Rust で書き直すにあたって、フロントエンドには Web フレームワークである「[axum](https://github.com/tokio-rs/axum)」を、フロントエンドとバックエンドの連携は gRPC で行われます。そのためのライブラリとして、「[tonic](https://github.com/hyperium/tonic)」を使いました。

## Index

- [フロントエンド](./frontend/0.frontend.md)
  1. [axum](./frontend/1.axum.md)
  1. [handler](./frontend/2.handler.md)
- [バックエンド](./backend/0.backend.md)
