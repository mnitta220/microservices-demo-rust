<p align="center">
<img src="/docs/rust/img/logo.png" width="520" alt="Online Boutique" />
</p>

# microservice-demo-rust

[English](./README.md)&nbsp;&nbsp;|&nbsp;&nbsp;Japanese

このプロジェクトは、Google が提供している **Online Boutique** ([microservices-demo](https://github.com/GoogleCloudPlatform/microservices-demo)) デモアプリケーションの一部を、[Rust](https://www.rust-lang.org/ja) で書き直したものです。  
ユーザーが商品を閲覧したり、カートに追加したり、購入したりできるウェブベースの E コマース アプリケーションです。[Kubernetes](https://kubernetes.io/) クラスター上で動作し、マイクロサービスが [gRPC](https://grpc.io/) で連携します。開発言語には、Go、C#、JavaScript、Java、Python が使われています。  
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
| [adservice](/src/adservice)                         | Java           | **Rust** | 指定されたコンテキストの単語に基づいてテキスト広告を提供します。                                                                                     |

<br>

> このプロジェクトは、[microservices-demo](https://github.com/GoogleCloudPlatform/microservices-demo) の 2024 年 3 月 11 日時点のソースコードを fork して開発を行いました。それ以降に microservices-demo で行われた更新は取り込んでいません。

<br>

## 目的

私が [microservices-demo](https://github.com/GoogleCloudPlatform/microservices-demo) を Rust で書き直した理由は、Kubernetes クラスタで動作する Web システムを Rust で作るとどうなるのか試してみたかったからです。microservices-demo は、学習と検証をするには、ちょうど良い規模のプロジェクトでした。  
[axum](https://github.com/tokio-rs/axum) 、 [Tokio](https://tokio.rs/) 、[tonic](https://github.com/hyperium/tonic) などを使うのは初めてでしたたので、いろいろと調べるのに時間がかかりました。よりよいコーディングを求めて、何度も書き直しました。  
Rust は、私が一番好きな言語です。メモリ安全性とパフォーマンスに優れたシステムを作ることができます。特に、GC（ガベージコレクション）が実行されない点が気に入っています。今回の開発でも、期待通りの満足する結果が得られました。フロントエンド開発においても、非常に適した言語であり、開発効率や生産性の面でも優れているという確信を持つことができました。  
実際のシステム開発では、Rust が使用されるケースは、まだ少ないようです。今後、様々なシステム開発で、Rust の採用が増えることを願っています。  
今回行った実装には、改善できる点がまだ多いだろうと思います。改善点やご意見がありましたら、ぜひお寄せください。

<br>

## Screenshots

| Home Page                                                                                                             | Checkout Screen                                                                                                        |
| --------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| [![Screenshot of store homepage](/docs/img/online-boutique-frontend-1.png)](/docs/img/online-boutique-frontend-1.png) | [![Screenshot of checkout screen](/docs/img/online-boutique-frontend-2.png)](/docs/img/online-boutique-frontend-2.png) |

## Quickstart

### Rust 開発環境のセットアップ

[Rust での開発](/docs/rust/jp/1.development/1-0.development.md)

### Docker Desktop での実行

1. [Git](https://git-scm.com/)、[Docker Desktop](https://www.docker.com/products/docker-desktop/)、[Skaffold](https://skaffold.dev/docs/install/)をインストールします。

1. Docker Desktop の Settings - Kubernetes で “Enable Kubernetes” を選択します。

1. Clone the repository.

   ```sh
   git clone https://github.com/mnitta220/microservices-demo-rust.git
   cd microservices-demo-rust/
   ```

1. `kubectl get nodes` を実行して、docker-desktop コントロールプレーンに接続されていることを確認してください。

1. `skaffold run` を実行してください（初回は遅く、約 20 分かかる場合があります）。
   これによりアプリケーションがビルドされ、デプロイされます。コードをリファクタリングする際に自動的にイメージを再ビルドする必要がある場合は、`skaffold dev` コマンドを実行してください。

1. `kubectl get pods` を実行して、Pod が準備完了して実行中であることを確認してください。

1. `kubectl port-forward deployment/frontend 8080:8080` を実行して、ポートをフロントエンド サービスに転送してください。

1. http://localhost:8080 にアクセスして、ウェブフロントエンドにアクセスしてください。

<br>

- Cleanup

  - `skaffold run` コマンドでアプリケーションをデプロイした場合、`skaffold delete` を実行してデプロイされたリソースをクリーンアップできます。

### GKE での実行

1. Ensure you have the following requirements:

   - [Google Cloud project](https://cloud.google.com/resource-manager/docs/creating-managing-projects#creating_a_project).
   - Shell environment with `gcloud`, `git`, and `kubectl`.

2. Clone the repository.

   ```sh
   git clone https://github.com/mnitta220/microservices-demo-rust.git
   cd microservices-demo-rust/
   ```

3. Set the Google Cloud project and region and ensure the Google Kubernetes Engine API is enabled.

   ```sh
   export PROJECT_ID=<PROJECT_ID>
   export REGION=us-central1
   gcloud services enable container.googleapis.com \
     --project=${PROJECT_ID}
   ```

   Substitute `<PROJECT_ID>` with the ID of your Google Cloud project.

4. Confirm the services have been enabled for your project.

   ```sh
   gcloud services list --enabled --project=${PROJECT_ID}
   ```

5. Create a GKE cluster and get the credentials for it.

   ```sh
   gcloud container clusters create-auto online-boutique \
     --project=${PROJECT_ID} --region=${REGION}
   ```

   Creating the cluster may take a few minutes.

6. Deploy Online Boutique to the cluster.

   ```sh
   kubectl apply -f ./release/kubernetes-manifests.yaml
   ```

7. Wait for the pods to be ready.

   ```sh
   kubectl get pods
   ```

   After a few minutes, you should see the Pods in a `Running` state:

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

8. Access the web frontend in a browser using the frontend's external IP.

   ```sh
   kubectl get service frontend-external | awk '{print $4}'
   ```

   Visit `http://EXTERNAL_IP` in a web browser to access your instance of Online Boutique.

9. Congrats! You've deployed the default Online Boutique. To deploy a different variation of Online Boutique (e.g., with Google Cloud Operations tracing, Istio, etc.), see [Deploy Online Boutique variations with Kustomize](#deploy-online-boutique-variations-with-kustomize).

10. Once you are done with it, delete the GKE cluster.

```sh
gcloud container clusters delete online-boutique \
  --project=${PROJECT_ID} --region=${REGION}
```

Deleting the cluster may take a few minutes.

## Use Terraform to provision a GKE cluster and deploy Online Boutique

The [`/terraform` folder](/terraform) contains instructions for using [Terraform](https://www.terraform.io/intro) to replicate the steps from [**Quickstart (GKE)**](#quickstart-gke) above.

## Other deployment variations

- **Istio/Anthos Service Mesh**: [See these instructions.](/kustomize/components/service-mesh-istio/README.md)
- **non-GKE clusters (Minikube, Kind)**: see the [Development Guide](/docs/development-guide.md)

## Deploy Online Boutique variations with Kustomize

The [`/kustomize` folder](/kustomize) contains instructions for customizing the deployment of Online Boutique with different variations such as:

- integrating with [Google Cloud Operations](/kustomize/components/google-cloud-operations/)
- replacing the in-cluster Redis cache with [Google Cloud Memorystore (Redis)](/kustomize/components/memorystore), [AlloyDB](/kustomize/components/alloydb) or [Google Cloud Spanner](/kustomize/components/spanner)
- etc.

## Development

See the [Development guide](/docs/development-guide.md) to learn how to run and develop this app locally.

See the [Rust での書き直し](/docs/rust/jp/index.md)

<br>

## パフォーマンス比較

http://localhost:8080/

|        | Original | Rewrote in Rust |
| ------ | -------: | --------------: |
| 1 回目 |     52ms |             9ms |
| 2 回目 |     13ms |            10ms |
| 3 回目 |     15ms |            11ms |

http://localhost:8080/product/OLJCESPC7Z

|        | Original | Rewrote in Rust |
| ------ | -------: | --------------: |
| 1 回目 |     17ms |            13ms |
| 2 回目 |     15ms |            14ms |
| 3 回目 |     15ms |            15ms |
