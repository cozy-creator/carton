# Carton

> This project is in active development. So, it's features are likely to change or break in the future.

Carton is a Sui Move smart contract development tool. You can use it to create, test, build and publish your Move pcakges. It also support the management of multiple packages in a project workspace.

## Installation

### Prerequisites

To install Carton on your machine, you might need to install some prerequisites and tools if they are not already installed. The prerequisites are:

- cURL
- Rust and Cargo
- Git CLI
- CMake
- libssl-dev
- libclang-dev
- Brew
- C++ build tools
- LLVM Compile

The prerquisites are the same as Sui's, So if you have Sui installed on your machine already you can skip this step.

If you do not have Sui installed, use the like below to learn how to install them:

https://docs.sui.io/build/install#prerequisites

### Install Carton

After installing the prerequisites, you can proceed to install carton by running:

```sh
cargo install --git https://github.com/bytedeveloperr/carton.git
```

This will install the carton cli and once the installation is complete, you can test the installation by running the command below to check it's version:

```sh
carton -v
```
