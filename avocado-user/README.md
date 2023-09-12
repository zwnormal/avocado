# Avocado User - User Authentication and Authorisation Microservice

The purpose of Avocado User microservice is to build a support microservice for the avocado platform, the current functions (or current planned functions) include adding/updating/disabling/deleting user, authenticate user by email and password and issue jwt access token & refresh token, jwt token and user verification, as well as permissions control based per command level.

## Build and Run
1. The code is written by Rust, so need to install rust toolchain firstly. Please follow the steps at https://www.rust-lang.org/tools/install.
2. Rust crate [tonic](https://github.com/hyperium/tonic) is used to build grpc endpoints, so please follow the Dependencies section of the README on the repo to install tools from compiling proto IDL.
3. Clone the code as well as its submodules:
    ```commandline
    git clone --recurse-submodules git@github.com:zwnormal/avocado-user.git
    ```
4. To run the code, it requires a config file called `config.yaml`, an example has been provided as `config.yaml.sample`. Please copy and rename it to `config.yaml`:
    ```commandline
    cp config.user.yaml.sample config.user.yaml        
    ```
5. Install the cargo bunyan to view the formatted log output:
   ```commandline
   cargo install bunyan   
   ```
6. To run both unit tests and integration tests:
    ```commandline
    TEST_LOG=1 cargo test | bunyan
    ```
    The flag `TEST_LOG` enables the tracing when running tests, and `bunyan` shows formatted log output to the `stdout`. Please check the `main.rs` for details about how `tracing` is configured. Because the configuration of `tracing` is reusable to multiple microservice, currently its configuration functions are in another repo [avocado-base](https://github.com/zwnormal/avocado-base).
7. To run the grpc server:
    ```commandline
    cargo run | bunyan
    ```
   After the server starts, API test tools like `Postman` can be used to test the grpc endpoints. Currently check the `main.py` to see which port the server is running, and check the proto IDL files under `proto` folder to see what endpoints available for now. Please note the IDL files under `proto` folder belongs to other repos, for example, `user` IDLs belongs to [avocado-user-proto](https://github.com/zwnormal/avocado-user-proto). When project grows, it is expected each microservice as its own separate proto repo to facilitate decoupling and also composition.