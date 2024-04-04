English&nbsp;&nbsp;|&nbsp;&nbsp;[Japanese](../jp/index.md)

# Rewrote in Rust

This application has the architecture shown in the following diagram.

![Architecture Diagram](/docs/img/architecture-diagram.png)

Each service acts as a Kubernetes node. gRPC is used for communication between services.
The **frontend**, **productcatalog**, **cart**, and **ad** were rewritten in Rust.  
When rewriting it in Rust, I used the web framework [axum](https://github.com/tokio-rs/axum) for the frontend and [tonic](https://github.com/hyperium/tonic) for the gRPC library.

In rewriting, rather than simply replacing the original source code with Rust, there were some parts where I rewrote the code to be more efficient and omitted time-consuming implementation.

## Index

- [1. Development in Rust](./1.development/1-0.development.md)
- [2. Frontend](./2.frontend/2-0.frontend.md)
  - [2-1. axum](./2.frontend/2-1.axum.md)
  - [2-2. Handler](./2.frontend/2-2.handler.md)
  - [2-3. Page](./2.frontend/2-3.page.md)
  - [2-4. Model](./2.frontend/2-4.model.md)
  - [2-5. gRPC](./2.frontend/2-5.rpc.md)
  - [2-6. Component](./2.frontend/2-6.component.md)
- [3. Backend](./3.backend/3-0.backend.md)
  - [3-1. productcatalog service](./3.backend/3-1.productcatalog.md)
  - [3-2. cart service](./3.backend/3-2.cart.md)
  - [3-3. ad service](./3.backend/3-3.ad.md)

<table style="width: 90%; margin-top: 20px;">
<tr>
<td style="text-align: left"></td>
<td></td>
<td style="text-align: right"><a href="./1.development/1-0.development.md">1. Development in Rust&nbsp;&gt;</a></td>
</tr>
</table>
