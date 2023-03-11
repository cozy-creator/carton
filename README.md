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

If you do not have Sui installed, use the link below to learn how to install them:

https://docs.sui.io/build/install#prerequisites

### Install Carton

After installing the prerequisites, you can proceed to install carton by running:

```sh
cargo install --git https://github.com/bytedeveloperr/carton.git
```

This will install the carton cli and once the installation is complete, you can test the installation by running the command below to check it's version:

```sh
carton --version
```

Also, you can view the list of available commands using:

```sh
carton --help
```

### Carton manifest

Every Carton project must have a `Carton.toml` at the project root directory, This file is the manifest which contains metadata that are required to run the project using the carton cli.

A basic `Carton.toml` file looks like this:

```toml
[provider]
address = "0x2a1b6f57961805c46c21166712aad57095808b18"
env = "devnet"
config = "~/.sui/sui_config/client.yaml"

[envs]
devnet = { url = "https://fullnode.devnet.sui.io:443/" }
testnet = { url = "https://fullnode.testnet.sui.io:443/" }
```

Now, let's go through some of the sections of the `Carton.toml` file

### Provider [provider]

This is a required section and is used to specify the Sui provider connection details such as address, env and config. The expected fields for this section are:

- address: the address to be used when running commands that involve interaction with the blockchain network e.g publish, javascript tests(coming soon) etc.
- env: the network environment to be used
- config: path to the sui client config file

Note: the address must corresspond to one of keys present in the keystore file specified inside the sui client config file

### Envs [envs]

This is an optional section which contains the network environments that can be used in the project. e.g

```toml
[envs]
env-name = { url = "https://suienvrpc.com" }
```

Other possible sections are:

- Workspace

## Create a project

To create a project with Carton, you can use the command below:

```sh
carton create <project_name>
```

This will create a new project with the supplied project name and will also create Carton manifest file named `Carton.toml` in the project directory.

## Initialize Carton (in an existing project or directory)

If you have an existing project or directory, you can initialize Carton in the project using the command below:

```sh
carton init
```

The above command will fail if a `Carton.toml` file already exist in the project or directory. To prevent this failure, you can supply the `--force` arg which will replace the existing `Carton.toml` with a new one.

```sh
carton init --force
```

## Working with multiple packages (Workspace)

If you are working on a project that involves multiple move packages, You have to keep switching between multiple directories to work with individual packages which is very tiring and inefficient.

Carton allows you to work with multiple packages with ease, without going through the pain of managing them altogether. You can use this feature by specifying the workspace members in the `[workspace]` section to your Carton.toml file. e.g

```toml
[workspace]
members = ["package1", "package2", "packages/package3"]
```

You can also add members using the glob pattern, e.g

```toml
[workspace]
members = ["packages/*", "src/**/pacakges"]
```

### Specifying packages in a workspace

When working in a workspace and you want to run a command in a particular package, you can specify the package using the name specified in the package's `Move.toml` file. e.g

```sh
carton command --package PackageName
```

The command will fail if the package does not exist or is not registered.

## Building packages

To build a package using Carton in a non-workspace project you can run:

```sh
carton build
```

and if you are in a workspace, you should specify the package name:

```sh
carton build --package PackageName
```

An example output of the build command is:

```sh
UPDATING GIT DEPENDENCY https://github.com/MystenLabs/sui.git
UPDATING GIT DEPENDENCY https://github.com/MystenLabs/sui.git
UPDATING GIT DEPENDENCY https://github.com/MystenLabs/sui.git
UPDATING GIT DEPENDENCY https://github.com/MystenLabs/sui.git
INCLUDING DEPENDENCY Sui
INCLUDING DEPENDENCY MoveStdlib
BUILDING PackageName
```

## Testing packages

To test a package using Carton in a non-workspace project you can run:

```sh
carton test
```

and if you are in a workspace, you should specify the package name:

```sh
carton test --package PackageName
```

An example output of the test command is:

```sh
UPDATING GIT DEPENDENCY https://github.com/MystenLabs/sui.git
UPDATING GIT DEPENDENCY https://github.com/MystenLabs/sui.git
UPDATING GIT DEPENDENCY https://github.com/MystenLabs/sui.git
UPDATING GIT DEPENDENCY https://github.com/MystenLabs/sui.git
INCLUDING DEPENDENCY Sui
INCLUDING DEPENDENCY MoveStdlib
BUILDING PackageName
Running Move unit tests
[ PASS    ] 0x0::module1_tests::test_something
[ PASS    ] 0x0::module1_tests::test_anotherthing
[ PASS    ] 0x0::module2_tests::some_other_test
Test result: OK. Total tests: 3; passed: 3; failed: 0
```

## Publishing packages

To publish a package using Carton in a non-workspace project you can run:

```sh
carton publish
```

and if you are in a workspace, you should specify the package name:

```sh
carton publish --package PackageName
```

An example output of the publish command is:

```sh
UPDATING GIT DEPENDENCY https://github.com/MystenLabs/sui.git
UPDATING GIT DEPENDENCY https://github.com/MystenLabs/sui.git
UPDATING GIT DEPENDENCY https://github.com/MystenLabs/sui.git
UPDATING GIT DEPENDENCY https://github.com/MystenLabs/sui.git
INCLUDING DEPENDENCY Sui
INCLUDING DEPENDENCY MoveStdlib
BUILDING PackageName
Skipping dependency verification

Transaction Digest: CaWn5Brc1GVbibgrRyPVSxezw42FG2imysPjHzbCwzDa
Package ID: 0x15149c8ad353d064613dc5bd1b6a0416539b4dff

Objects Created
  - Object ID: 0x15149c8ad353d064613dc5bd1b6a0416539b4dff
    Owner: Immutable
    Digest: o#7wG8utjMSowzZ3dHe6ZD8tVneQ8Nl8NFI/bRJGGoQdo=
    Version: 1

Objects Mutated
  - Object ID: 0x331d6609bafb58d5fa12bd3d1d63cd72ee95170f
    Owner: Account Address ( 0x2a1b6f57961805c46c21166712aad57095808b18 )
    Digest: o#UPzoEnGonCBk1vGxOrYLzlhSNa6cDTt8KyWUyfvntaU=
    Version: 7612

PUBLISHED SUCCESSFULLY 🥳
```

## Thank you!

### Found a bug?

Please raise an issue providing some information about it [here](https://github.com/bytedeveloperr/carton/issues/new).
