# KmodtRust - Integrity for Kernel Modules

![License](https://img.shields.io/github/license/puru1761/kcapi)

This repository contains the sources for the ``kmodtrust`` utility, which can
be used to sign and verify Linux Kernel Modules. This is a `rust` based utility
which can be installed as a `suid` binary so that private keys need not be
exposed to all users needing to perform kernel module signing.

## Pre-requisites

In order to build and use this utility, the following conditions need
to be met:

1. Install the `rust` toolchain using the instructions at [rustup](https://rustup.rs/)

## Build

Build the utility using `make`

```shell
make
```

## Installation

To install it to `${PWD}`, use:

```shell
make install
```

To install `kmodtrust` to a custom root, modify the `INSTALL_PREFIX` variable:

```shell
make install INSTALL_PREFIX=/path/to/install-root
# For example
make install INSTALL_PREFIX=/usr/local
```

## Test

To build the test module under the `test` directory, ensure that you have
the kernel development headers installed.

Once, you are sure that you can build kernel modules on your machine, build
the kernel module and generate keys for signing:

```shell
cd test
make
```

Once the required artifacts are generated, you can sign the generated ``hello.ko``
using:

```shell
../target/release/kmodtrust sign \
    --key certs/kmodtrust_key.pem \
    --x509-cert certs/kmodtrust.x509 \
    module/hello.ko
```

## Usage

The `kmodtrust` utility has the following help text:

```
kmodtrust 0.1.0
Puru Kulkarni <puruk@protonmail.com>
Linux Kernel Module Integrity

USAGE:
    kmodtrust [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    sign    Sign a Linux Kernel Module
```

Help text for each subcommand can be further seen by passing the --help
flag to the subcommand:

```
kmodtrust sign --help
```

To sign a kernel module the following incantation can be done:

```
kmodtrust sign --key <KEY> --x509-cert <X509_FILE> <MODULE>
```

For example,

```
kmodtrust sign --key /path/to/privatekey.pem --x509-cert /path/to/cert.x509 /path/to/module.ko
```

Author(s)

* Purushottam A. Kulkarni <<puruk@protonmail.com>>