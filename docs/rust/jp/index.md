[English](../en/index.md)&nbsp;&nbsp;|&nbsp;&nbsp;Japanese

# Rust での書き直し

このアプリケーションは、次の図のようなアーキテクチャーになっています。

![Architecture Diagram](/docs/img/architecture-diagram.png)

この図の各サービスが、Kubernetes のノードとして動作します。サービス間の通信には gRPC が使われます。  
Rust で書き直しを行ったのは、frontend、productcatalog、cart、ad です。  
Rust で書き直すにあたって、フロントエンドには Web フレームワークである「[axum](https://github.com/tokio-rs/axum)」を、 gRPC のライブラリには「[tonic](https://github.com/hyperium/tonic)」を使いました。

## Index

- [フロントエンド](./frontend/0.frontend.md)
  1. [axum](./frontend/1.axum.md)
  1. [ハンドラー](./frontend/2.handler.md)
  1. [ページ](./frontend/3.page.md)
  1. [モデル](./frontend/4.model.md)
  1. [gRPC](./frontend/5.rpc.md)
  1. [PageProps](./frontend/6.page-props.md)
  1. [コンポーネント](./frontend/7.component.md)
- [バックエンド](./backend/0.backend.md)
  1. [productcatalog サービス](./backend/1.productcatalog.md)
