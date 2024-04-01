# Backend

フロントエンドを Rust で書き直すに際して、以下の方針で実装しました。

1. オリジナルでは HTML のテンプレートを使用しているが、書き直しではテンプレートを使用しない。
2. 画面をコンポーネントに分割し、各コンポーネントが HTML のタグを出力する。

ソースコードは、以下のような構成にしました。

| フォルダ   | 役割                                                                |
| ---------- | ------------------------------------------------------------------- |
| components | 画面の構成要素に応じた部品。HTML タグの出力を行う。                 |
| model      | データの取得と保持を行う。                                          |
| pages      | 画面の制御を行う。                                                  |
| rpc        | gRPC により、他のサービスと連携してデータの取得・登録・更新を行う。 |

MVC で言うと、components が View、model が Model、pages が Controller に該当します。

## Index

- [frontend](./frontend.md)
  - [components](./components.md)
- [backend](./backend.md)
