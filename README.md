# MySo Sdk

A rust Sdk for integrating with the [MySo blockchain](https://docs..io/).

> [!NOTE]
> This is project is under development and many features may still be under
> development or missing.

## Overview

This repository contains a collection of libraries for integrating with the MySo blockchain.

A few of the project's high-level goals are as follows:

* **Be modular** - user's should only need to pay the cost (in terms of dependencies/compilation time) for the features that they use.
* **Be light** - strive to have a minimal dependency footprint.
* **Support developers** - provide all needed types, abstractions and APIs to enable developers to build robust applications on MySo.
* **Support wasm** - where possible, libraries should be usable in wasm environments.

## Crates

In an effort to be modular, functionality is split between a number of crates.

* [`-sdk-types`](crates/-sdk-types)
    [![-sdk-types on crates.io](https://img.shields.io/crates/v/-sdk-types)](https://crates.io/crates/-sdk-types)
    [![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/-sdk-types)
    [![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mystenlabs.github.io/-rust-sdk/_sdk_types/)
* [`-crypto`](crates/-crypto)
    [![-crypto on crates.io](https://img.shields.io/crates/v/-crypto)](https://crates.io/crates/-crypto)
    [![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/-crypto)
    [![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mystenlabs.github.io/-rust-sdk/_crypto/)
* [`-graphql-client`](crates/-crypto)
    [![-graphql-client on crates.io](https://img.shields.io/crates/v/-graphql-client)](https://crates.io/crates/-graphql-client)
    [![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/-graphql-client)
    [![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mystenlabs.github.io/-rust-sdk/-graphql-client/)

## License

This project is available under the terms of the [Apache 2.0 license](LICENSE).
