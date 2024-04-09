<p align="center">
<img src="/docs/rust/img/logo.png" width="520" alt="Online Boutique" />
</p>

# microservice-demo-rust

[English](./README.md)&nbsp;&nbsp;|&nbsp;&nbsp;Japanese

このプロジェクトは、Google が提供している **Online Boutique** ([microservices-demo](https://github.com/GoogleCloudPlatform/microservices-demo)) デモアプリケーションの一部を、[Rust](https://www.rust-lang.org/ja) で書き直したものです。  
ユーザーが商品を閲覧したり、カートに追加したり、購入したりできる Web ベースの E コマース アプリケーションです。[Kubernetes](https://kubernetes.io/) クラスター上で動作し、マイクロサービスが [gRPC](https://grpc.io/) で連携します。開発言語には、Go、C#、JavaScript、Java、Python が使われています。  
11 個のマイクロサービスで構成されています。私の方で、この中の 4 個のサービスを Rust で書き直しました。  
次の表で、`書き直し` 欄に "**Rust**" と書かれているサービスを Rust で書き直しました。"**Rust**" と書かれていないサービスはオリジナルのままです。

| サービス                                            | オリジナル言語 | 書き直し | 説明                                                                                                                                                 |
| --------------------------------------------------- | -------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| [frontend](/src/frontend)                           | Go             | **Rust** | ウェブサイトを提供するための HTTP サーバを公開します。サインアップ/ログインは不要で、すべてのユーザーに対して自動的にセッション ID を生成します。    |
| [cartservice](/src/cartservice)                     | C#             | **Rust** | ユーザーのショッピングカート内の商品を Redis に保存して、それを取得します。                                                                          |
| [productcatalogservice](/src/productcatalogservice) | Go             | **Rust** | JSON ファイルから商品の一覧を提供し、商品を検索して個々の商品を取得する機能を提供します。                                                            |
| [currencyservice](/src/currencyservice)             | JavaScript     |          | ある通貨の金額を別の通貨に変換します。ヨーロッパ中央銀行から取得した実際の値を使用します。これは最も高い QPS（1 秒あたりのクエリ数）のサービスです。 |
| [paymentservice](/src/paymentservice)               | JavaScript     |          | 与えられたクレジットカード情報（モック）と金額で課金し、トランザクション ID を返します。                                                             |
| [shippingservice](/src/shippingservice)             | Go             |          | ショッピングカートに基づいて送料の見積もりを提供します。アイテムを指定された住所に発送します。（モック）                                             |
| [emailservice](/src/emailservice)                   | Python         |          | ユーザーに注文確認のメールを送信します。（モック）                                                                                                   |
| [checkoutservice](/src/checkoutservice)             | Go             |          | ユーザーのカートを取得し、注文を準備し、支払い、配送、およびメール通知を調整します。                                                                 |
| [recommendationservice](/src/recommendationservice) | Python         |          | カートに入っている商品に基づいて、他の商品をお勧めします。                                                                                           |
| [adservice](/src/adservice)                         | Java           | **Rust** | ランダムなテキスト広告を提供します。                                                                                                                 |

<br>

> このプロジェクトは、[microservices-demo](https://github.com/GoogleCloudPlatform/microservices-demo) の 2024 年 3 月 11 日時点のソースコードを fork して開発を行いました。それ以降に microservices-demo で行われた更新は取り込んでいません。

<br>

## 目的

私が [microservices-demo](https://github.com/GoogleCloudPlatform/microservices-demo) を Rust で書き直した目的は、Kubernetes クラスターで動作する Web システムを Rust で作るとどうなるのか試してみたかったからです。microservices-demo は、学習と検証をするには、ちょうど良い規模のプロジェクトでした。  
Rust の実装では、フロントエンドには Web フレームワークである [axum](https://github.com/tokio-rs/axum) を、 gRPC のライブラリには [tonic](https://github.com/hyperium/tonic) を使いました。  
フロントエンドでは、画面をコンポーネントに分割して、コンポーネントが HTML を生成するという方法を採りました。これは、[React](https://react.dev/) に触発されたものです。  
サーバーサイドで HTML を生成していながらも、React のようなコンポーネント指向であるというプログラムにしました。そうした実装内容の説明は、[Rust での書き直し](/docs/rust/jp/index.md)をご覧ください。  
今回行った実装には、改善できる点がまだ多いだろうと思います。改善点やご意見がありましたら、ぜひお寄せください。

<br>

## Screenshots

| Home Page                                                                                                             | Checkout Screen                                                                                                        |
| --------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| [![Screenshot of store homepage](/docs/img/online-boutique-frontend-1.png)](/docs/img/online-boutique-frontend-1.png) | [![Screenshot of checkout screen](/docs/img/online-boutique-frontend-2.png)](/docs/img/online-boutique-frontend-2.png) |

## Quickstart

### Rust 開発環境のセットアップ

このプロジェクトを、Rust で開発する方法は、[Rust での開発](/docs/rust/jp/1.development/1-0.development.md) をご覧ください。

### Docker Desktop での実行

1. [Git](https://git-scm.com/)、[Docker Desktop](https://www.docker.com/products/docker-desktop/)、[Skaffold](https://skaffold.dev/docs/install/)をインストールします。

1. Docker Desktop の Settings - Kubernetes で “Enable Kubernetes” を選択します。

1. Clone the repository.

   ```sh
   git clone https://github.com/mnitta220/microservices-demo-rust.git
   cd microservices-demo-rust/
   ```

1. `kubectl get nodes` を実行して、docker-desktop コントロールプレーンが起動していることを確認してください。

1. `skaffold run` を実行してください（初回は遅く、約 30 分かかる場合があります）。
   これによりアプリケーションがビルドされ、デプロイされます。コードをリファクタリングする際に自動的にイメージを再ビルドする必要がある場合は、`skaffold dev` コマンドを実行してください。

1. `kubectl get pods` を実行して、Pod が準備完了して実行中であることを確認してください。

1. `kubectl port-forward deployment/frontend 8080:8080` を実行して、ポートをフロントエンド サービスに転送してください。

1. http://localhost:8080 にアクセスして、ウェブフロントエンドにアクセスしてください。

<br>

- クリーンアップ

  - `skaffold run` コマンドでアプリケーションをデプロイした場合、`skaffold delete` を実行してデプロイされたリソースをクリーンアップできます。

### GKE での実行

1. 次の要件を満たしていることを確認してください。

   - [Google Cloud project](https://cloud.google.com/resource-manager/docs/creating-managing-projects#creating_a_project).
   - `gcloud`、 `git`、 `kubectl` コマンド。

2. リポジトリをクローンしてください。

   ```sh
   git clone https://github.com/mnitta220/microservices-demo-rust.git
   cd microservices-demo-rust/
   ```

3. Google Cloud プロジェクトとリージョンを設定し、Google Kubernetes Engine API が有効になっていることを確認します。

   ```sh
   export PROJECT_ID=<PROJECT_ID>
   export REGION=us-central1
   gcloud services enable container.googleapis.com \
     --project=${PROJECT_ID}
   ```

   `<PROJECT_ID>` を Google Cloud プロジェクトの ID に置き換えます。

4. サービスがプロジェクトで有効になっていることを確認します。

   ```sh
   gcloud services list --enabled --project=${PROJECT_ID}
   ```

5. GKE クラスタを作成し、その認証情報を取得します。

   ```sh
   gcloud container clusters create-auto online-boutique \
     --project=${PROJECT_ID} --region=${REGION}
   ```

   クラスターの作成には数分かかる場合があります。

6. Online Boutique をクラスターにデプロイします。

   ```sh
   kubectl apply -f ./release/kubernetes-manifests.yaml
   ```

7. Pod の準備ができるまで待ちます。

   ```sh
   kubectl get pods
   ```

   数分後、ポッドが `Running` 状態になるはずです。

   ```
   NAME                                     READY   STATUS    RESTARTS   AGE
   adservice-76bdd69666-ckc5j               1/1     Running   0          2m58s
   cartservice-66d497c6b7-dp5jr             1/1     Running   0          2m59s
   checkoutservice-666c784bd6-4jd22         1/1     Running   0          3m1s
   currencyservice-5d5d496984-4jmd7         1/1     Running   0          2m59s
   emailservice-667457d9d6-75jcq            1/1     Running   0          3m2s
   frontend-6b8d69b9fb-wjqdg                1/1     Running   0          3m1s
   paymentservice-68596d6dd6-bf6bv          1/1     Running   0          3m
   productcatalogservice-557d474574-888kr   1/1     Running   0          3m
   recommendationservice-69c56b74d4-7z8r5   1/1     Running   0          3m1s
   redis-cart-5f59546cdd-5jnqf              1/1     Running   0          2m58s
   shippingservice-6ccc89f8fd-v686r         1/1     Running   0          2m58s
   ```

8. フロントエンドの外部 IP を使用して、ブラウザーで Web フロントエンドにアクセスします。

   ```sh
   kubectl get service frontend-external | awk '{print $4}'
   ```

   Web ブラウザで `http://EXTERNAL_IP` にアクセスし、Online Boutique のインスタンスにアクセスします。

9. おめでとうございます！ デフォルトの Online Boutique がデプロイされました。 Online Boutique の別のバリエーション (例: Google Cloud Operations トレース、Istio など) をデプロイするには、[Deploy Online Boutique variations with Kustomize](#deploy-online-boutique-variations-with-kustomize) を参照してください。

10. 完了したら、GKE クラスタを削除します。

```sh
gcloud container clusters delete online-boutique \
  --project=${PROJECT_ID} --region=${REGION}
```

クラスターの削除には数分かかる場合があります。

<br>

> ソースコードを更新して、それを GKE にデプロイする場合は、そのサービスの Docker イメージを次のコマンドでビルドし、<br>`docker image build -t <image-name>:<tag-name> .`<br>お使いのレジストリに push し、<br>`docker image push <image-name>:<tag-name>`<br>[/release/kubernetes-manifests.yaml](/release/kubernetes-manifests.yaml) の `image:` を書き換えてください。たとえば、frontend のイメージを更新したい場合は、<br>`image: masahironitta/microservices-demo-rust-frontend:0.1.0`<br>を書き換えてください。

## Terraform を使用して GKE クラスタをプロビジョニングし、Online Boutique をデプロイする

[`/terraform`](/terraform) フォルダーには、[Terraform](https://www.terraform.io/intro) を使用して [**クイックスタート (GKE)**](#quickstart-gke) の手順を複製する手順が含まれています。

## その他のデプロイバリエーション

- **Istio/Anthos Service Mesh**: [これらの手順](/kustomize/components/service-mesh-istio/README.md) を参照してください。
- **non-GKE clusters (Minikube, Kind)**: [開発ガイド](/docs/development-guide.md)をご覧ください。

## Kustomize を使用して Online Boutique のバリエーションをデプロイする

[`/kustomize`](/kustomize)フォルダーには、次のようなさまざまなバリエーションで Online Boutique のデプロイメントをカスタマイズするための手順が含まれています。

- [Google Cloud Operations](/kustomize/components/google-cloud-operations/) との統合
- クラスタ内 Redis キャッシュを [Google Cloud Memorystore (Redis)](/kustomize/components/memorystore)、 [AlloyDB](/kustomize/components/alloydb) または [Google Cloud Spanner](/kustomize/components/spanner) に置き換える。
- など

## 開発

このプロジェクトを、ローカル環境の PC で開発する場合の方法は、[Rust での書き直し](/docs/rust/jp/index.md)をご覧ください。

オリジナルの [Development guide](/docs/development-guide.md) も併せてご覧ください。

<br>

## パフォーマンス比較

私のローカル PC で `skaffold run` で起動し、[http://localhost:8080/](http://localhost:8080/)　にアクセスした応答時間を VSCode の Thunder Client 拡張機能で計測しました。  
オリジナルと Rust での書き換えで、それぞれ 3 回計測した結果が以下の通りです。

|        | オリジナル | Rust での書き換え |
| ------ | ---------: | ----------------: |
| 1 回目 |       52ms |              11ms |
| 2 回目 |       13ms |              10ms |
| 3 回目 |       15ms |              12ms |
