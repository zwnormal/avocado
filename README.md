# Avocado

The purpose of experimental project Avocado is to explore the new way of building application based on modern practices. The vision of the project is to build large cluster of standard components, or we can call them microservices, and then users of those components can combine them together to build any kind of system.

## Build and Run
1. The code is written by Rust, so need to install rust toolchain firstly. Please follow the steps at https://www.rust-lang.org/tools/install.
2. Rust crate [tonic](https://github.com/hyperium/tonic) is used to build grpc endpoints, so please follow the Dependencies section of the README on the repo to install tools for compiling proto IDL.
3. To run both unit tests and integration tests:
    ```commandline
    cargo test --workspace
    ```
4. The `avocado-user` component acts as an independent microservice for user authenticate and authorisation. To start `avocado-user` component:
    ```commandline
    cargo run --bin avocado-user
    ```
   Once started, it already has a demo admin created with username `admin@avocado.com` and password `kIxv4NomLT0WwGKF`;
5. The `avocado-crm` acts as a gateway for providing custom relationship management BFF, so it is not a component, instead, it supposes to be a CRM system built by existing component.
   ```commandline
   cargo run --bin avocado-crm
   ```
The `avocado-crm` depends on `avocado-user`, after running both, the backend of CRM is ready for user to login.

## Things to Do

There are still several important things to do to complete the infrastructure:
- [ ] Observability is important for microservice, need to attach a request id and span it across the whole request so all relevant logging can be linked together in one service. Also, [W3C Tracing Context](https://www.w3.org/TR/trace-context/) needs to be implement so the logging can be even associated across multiple microservices.
- [ ] Needs to implement the Authorisation by using [Casbin](https://github.com/casbin/casbin-rs).
- [ ] Currently, the sample Domain and Command pattern has been implemented, the one missing is the Event handling. Event is also important for sync data between different microservices, as each microservice will have its own database.
- [ ] Docker and deployment script for deploying the project to k8s. Ideally, the project should be able to deploy to any cloud provider that support k8s, so NO cloud provider specially API should be called directly without a middle layer.
