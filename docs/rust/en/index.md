English&nbsp;&nbsp;|&nbsp;&nbsp;[Japanese](../jp/index.md)

# Rewrote in Rust

このアプリケーションは、次の図のようなアーキテクチャーになっています。

![Architecture Diagram](/docs/img/architecture-diagram.png)

この図の各サービスが、Kubernetes のノードとして動作します。サービス間の通信には gRPC が使われます。  
Rust で書き直しを行ったのは、frontend、productcatalog、cart、ad です。  
Rust で書き直すにあたって、フロントエンドには Web フレームワークである「[axum](https://github.com/tokio-rs/axum)」を、 gRPC のライブラリには「[tonic](https://github.com/hyperium/tonic)」を使いました。

## Index

- [フロントエンド](./frontend/0.frontend.md)
  1. [axum](./frontend/1.axum.md)
  1. [handler](./frontend/2.handler.md)
- [バックエンド](./backend/0.backend.md)
