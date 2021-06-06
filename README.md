# PC-TRANSACT
**A sample Hyperledger Sawtooth-Sabre application [PRODUCE-CONSUME](https://github.com/arsulegai/produce-consume)
making use of [Hyperledger Transact](https://github.com/hyperledger/transact)**

A sample application that is written in rust and compiled using Transact library.

## What is it?

The application can PRODUCE items and CONSUME items after that.

## Prerequisites

### Docker Based
The application can be run as docker container. It has been tested on following
version of the docker components.
* Docker version 19.03.4
* Docker compose version 1.24.1

### Native Environment
The application is tested using the `rustc --version` 1.40.0-nightly.

## How to run?

1. Build the `pc-transact`
..**Method 1 (Docker)**: Clone the repository, run the following command

```
$ git clone https://github.com/arsulegai/pc-transact
$ cd pc-transact
$ docker-compose up
```

Wait for the container to be up. Login to the container and run the commands.

..**Method 2 (Native)**: Build the library

```
$ git clone https://github.com/arsulegai/pc-transact
$ cd pc-transact
$ cargo build
```

2. Run the application

```
$ PRODUCE apple 10
```

## Contributing

This software is in development phase and is Apache 2.0 licensed. We accept
contributions via [GitHub](https://github.com/arsulegai/pc-transact) pull
requests.
Each commit must include a `Signed-off-by:` in the commit message
(`git commit -s`). This sign-off means you agree the commit satisfies the
[Developer Certificate of Origin (DCO)](https://developercertificate.org/).

## License
This software is licensed under the [Apache License Version 2.0](LICENSE)
software license.

&copy; Copyright 2019, Arun S M
