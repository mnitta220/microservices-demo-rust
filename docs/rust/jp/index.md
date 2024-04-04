[English](../en/index.md)&nbsp;&nbsp;|&nbsp;&nbsp;Japanese

# Rust での書き直し

このアプリケーションは、次の図のようなアーキテクチャーになっています。

![Architecture Diagram](/docs/img/architecture-diagram.png)

この図の各サービスが、Kubernetes のノードとして動作します。サービス間の通信には gRPC が使われます。  
Rust で書き直しを行ったのは、frontend、productcatalog、cart、ad です。  
Rust で書き直すにあたって、フロントエンドには Web フレームワークである [axum](https://github.com/tokio-rs/axum) を、 gRPC のライブラリには [tonic](https://github.com/hyperium/tonic) を使いました。

書き直しにあたっては、オリジナルのソースコードをそのまま Rust に置き換えるのではなく、効率の良いコーディングに書き換えたり、手間のかかる実装を省略したりした箇所もあります。

## Index

- [1. Rust での開発](./1.development/1-0.development.md)
- [2.フロントエンド](./2.frontend/2-0.frontend.md)
  - [2-1.axum](./2.frontend/2-1.axum.md)
  - [2-2.ハンドラー](./2.frontend/2-1.axum.md)
  - [2-3.ページ](./2.frontend/2-3.page.md)
  - [2-4.モデル](./2.frontend/2-4.model.md)
  - [2-5.gRPC](./2.frontend/2-5.rpc.md)
  - [2-6.コンポーネント](./2.frontend/2-6.component.md)
- [3.バックエンド](./3.backend/3-0.backend.md)
  - [3-1.productcatalog サービス](./3.backend/3-1.productcatalog.md)
  - [3-2.cart サービス](./3.backend/3-2.cart.md)
  - [3-3.ad サービス](./3.backend/3-3.ad.md)

<table style="width: 90%; margin-top: 20px;">
<tr>
<td style="text-align: left"></td>
<td>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</td>
<td style="text-align: right"><a href="./1.development/1-0.development.md">1. Rust での開発&nbsp;&gt;</a></td>
</tr>
</table>
