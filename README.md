# Avocado

The purpose of experimental project Avocado is to explore the new way of building application based on modern practices. The vision of the project is to build large cluster of standard components, or we can call them microservices, and then users of those components can combine them together to build any kind of system.

## Build and Run
1. The code is written by Rust, so need to install rust toolchain firstly. Please follow the steps at https://www.rust-lang.org/tools/install.
2. Rust crate [tonic](https://github.com/hyperium/tonic) is used to build grpc endpoints, so please follow the Dependencies section of the README on the repo to install tools for compiling proto IDL.
3. To run both unit tests and integration tests:
    ```commandline
    TEST_LOG=1 cargo test --workspace | bunyan
    ```
   The flag `TEST_LOG` enables the tracing when running tests, and `bunyan` shows formatted log output to the `stdout`, and it is not necessary if un-formatted log output is ok.
4. The `avocado-user` component acts as an independent microservice for user authenticate and authorisation (not implemented yet, and [Casbin](https://github.com/casbin/casbin-rs) is expected to provide infrastructure). To start `avocado-user` component:
    ```commandline
    cargo run --bin avocado-user | bunyan
    ```
   Once started, it already has a demo admin created with username `admin@avocado.com` and password `kIxv4NomLT0WwGKF`;
5. The `avocado-crm` acts as a gateway for providing custom relationship management BFF, so it is not a component, instead, it supposes to be a CRM system built by existing component.
   ```commandline
   cargo run --bin avocado-crm | bunyan
   ```
The `avocado-crm` depends on `avocado-user`, after running both, the backend of crm is ready for user to login.
