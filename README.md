<p align="center">
<img src="/docs/rust/img/logo.png" width="520" alt="Online Boutique" />
</p>

# microservices-demo-rust

English&nbsp;&nbsp;|&nbsp;&nbsp;[Japanese](./README-ja.md)

This project is a rewrite of parts of the **Online Boutique** ([microservices-demo](https://github.com/GoogleCloudPlatform/microservices-demo)) demo application provided by Google in [Rust](https://www.rust-lang.org/).  
It is a web-based e-commerce application that allows users to browse products, add them to a cart, and purchase them. It runs on a [Kubernetes](https://kubernetes.io/) cluster and the microservices communicate with each other using [gRPC](https://grpc.io/). The development languages used are Go, C#, JavaScript, Java, and Python.  
It consists of 11 microservices. I have rewritten 4 of these services in Rust.  
In the following table, services marked in the `Rewrote` column as "**Rust**" have been rewritten in Rust. Services not marked as "**Rust**" remain in their original state.

| Service                                             | Original<br>Language | Rewote   | Description                                                                                                                       |
| --------------------------------------------------- | -------------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------- |
| [frontend](/src/frontend)                           | Go                   | **Rust** | Exposes an HTTP server to serve the website. Does not require signup/login and generates session IDs for all users automatically. |
| [cartservice](/src/cartservice)                     | C#                   | **Rust** | Stores the items in the user's shopping cart in Redis and retrieves it.                                                           |
| [productcatalogservice](/src/productcatalogservice) | Go                   | **Rust** | Provides the list of products from a JSON file and ability to search products and get individual products.                        |
| [currencyservice](/src/currencyservice)             | JavaScript           |          | Converts one money amount to another currency. Uses real values fetched from European Central Bank. It's the highest QPS service. |
| [paymentservice](/src/paymentservice)               | JavaScript           |          | Charges the given credit card info (mock) with the given amount and returns a transaction ID.                                     |
| [shippingservice](/src/shippingservice)             | Go                   |          | Gives shipping cost estimates based on the shopping cart. Ships items to the given address (mock)                                 |
| [emailservice](/src/emailservice)                   | Python               |          | Sends users an order confirmation email (mock).                                                                                   |
| [checkoutservice](/src/checkoutservice)             | Go                   |          | Retrieves user cart, prepares order and orchestrates the payment, shipping and the email notification.                            |
| [recommendationservice](/src/recommendationservice) | Python               |          | Recommends other products based on what's given in the cart.                                                                      |
| [adservice](/src/adservice)                         | Java                 | **Rust** | Provides random text ads.                                                                                                         |

<br>

> This project was developed by forking the source code of [microservices-demo](https://github.com/GoogleCloudPlatform/microservices-demo) as of March 11, 2024. Updates made to microservices-demo after that date have not been incorporated.

<br>

## Purpose

The purpose of rewriting [microservices-demo](https://github.com/GoogleCloudPlatform/microservices-demo) in Rust was to see how a web system running on a Kubernetes cluster would perform in Rust. **microservices-demo** was just the right size project for learning and experimentation.  
For the Rust implementation, I used the [axum](https://github.com/tokio-rs/axum) web framework for the frontend and [tonic](https://github.com/hyperium/tonic) for the gRPC library.  
On the frontend, I adopted a method of splitting the screen into components, where each component generates HTML. This was inspired by [React](https://react.dev/). Despite generating HTML on the server-side, it became a program with a component-oriented approach similar to React. For more details on these implementations, please refer to [Rewriting in Rust](/docs/rust/en/index.md).  
I believe there are still many areas for improvement in this implementation. If you have any suggestions or feedback, please feel free to share them.

<br>

## Screenshots

| Home Page                                                                                                             | Checkout Screen                                                                                                        |
| --------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| [![Screenshot of store homepage](/docs/img/online-boutique-frontend-1.png)](/docs/img/online-boutique-frontend-1.png) | [![Screenshot of checkout screen](/docs/img/online-boutique-frontend-2.png)](/docs/img/online-boutique-frontend-2.png) |

## Quickstart

### Development in Rust

For information on how to develop this project in Rust, please see the [Development in Rust](/docs/rust/en/1.development/1-0.development.md).

### Running on Docker Desktop

1. Install [Git](https://git-scm.com/), [Docker Desktop](https://www.docker.com/products/docker-desktop/) and [Skaffold](https://skaffold.dev/docs/install/)

1. Select “Enable Kubernetes” in Docker Desktop Settings - Kubernetes

1. Clone the repository.

   ```sh
   git clone https://github.com/mnitta220/microservices-demo-rust.git
   cd microservices-demo-rust/
   ```

1. Run `kubectl get nodes` to verify docker-desktop control plane is running.

1. Run `skaffold run` (first time will be slow, it can take ~30 minutes).
   This will build and deploy the application. If you need to rebuild the images
   automatically as you refactor the code, run `skaffold dev` command.

1. Run `kubectl get pods` to verify the Pods are ready and running.

1. Run `kubectl port-forward deployment/frontend 8080:8080` to forward a port to the frontend service.

1. Navigate to http://localhost:8080 to access the web frontend.

<br>

- Cleanup

  - If you've deployed the application with `skaffold run` command, you can run
    `skaffold delete` to clean up the deployed resources.

### Running on GKE

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

<br>

> When updating the source code and deploying it to GKE, build the Docker image for that service with the following command:<br>`docker image build -t <image-name>:<tag-name> .`<br>Then push it to your registry with the following command:<br>`docker image push <image-name>:<tag-name>`<br>Replace the `image:` in [/release/kubernetes-manifests.yaml](/release/kubernetes-manifests.yaml) with the new image. For example, if you want to update the frontend image, replace:<br>`image: masahironitta/microservices-demo-rust-frontend:0.1.0`

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

The method for developing this project on a local PC is described in the [Rewriting in Rust](/docs/rust/en/index.md).

See also original [Development guide](/docs/development-guide.md).

<br>

## Performance comparison

I measured the response time by accessing [http://localhost:8080/](http://localhost:8080/) using the Thunder Client extension in VSCode after starting it with `skaffold run` on my local PC. The results of three measurements for both the original and Rust version are as follows:

|             | Original | Rust version |
| ----------- | -------: | -----------: |
| First time  |     52ms |         11ms |
| Second time |     13ms |         10ms |
| Third time  |     15ms |         12ms |
