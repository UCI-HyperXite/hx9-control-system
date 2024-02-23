# Pod Operation

This Rust package is for the main program to be run on the pod.
The program runs a finite-state machine to operate the pod components
and acts as a Socket.IO server to communicate with the control station.

## First-time Setup

Install cargo-watch

```shell
cargo install cargo-watch
```

Add the build target for the Raspberry Pi

```shell
rustup target add armv7-unknown-linux-gnueabihf
```

### Cross-Compilation

Install the compiler for the Raspberry Pi platform

#### macOS

A homebrew formula for macOS cross-compiler toolchains is available

```shell
brew tap messense/macos-cross-toolchains
brew install armv7-unknown-linux-gnueabihf
```

#### Windows/Linux

For both of these platforms, a different process is necessary.

**Prerequisites**

First, install [Docker](https://docs.docker.com/). You can either install
Docker Desktop or the more lightweight Docker Engine, which is only available
on Linux distributions.

**Please note that on Linux, non-sudo users need to be in the `docker` group or
use rootless Docker.** You can read more about adding yourself to the group
[here](https://docs.docker.com/engine/install/linux-postinstall/#manage-docker-as-a-non-root-user).

Next, install the [`cross`](https://github.com/cross-rs/cross) package by
running the below command in the `pod-operation/` directory.

```shell
cargo install cross --git https://github.com/cross-rs/cross
```

This is the library that will facilitate cross-compilation without much
configuration on our end. It requires Docker in order to function.

Ensure that `cross` is now on your `PATH`. Usually, it
is located in the `.cargo/bin` folder of your home directory, so if `cross` is not in your `PATH`, you can check this directory and add it to `PATH`.

Now run

```shell
cross build
```

and the program should build. On Linux, if an error appears about `GLIBC`,
you may need to install the `glibc` library. You can do this by running

```shell
# Ubuntu/Debian-based distributions
sudo apt install glibc

# Arch-based distributions
sudo pacman -S glibc
```

## Local Development

To locally run the development server with auto-reload

```shell
cargo watch -x run
```

## Building for Production

To build for production, use the `--release` option

```shell
cargo build --release
```

This will compile the project to
`target/armv7-unknown-linux-gnueabihf/release/pod-operation`
which can be run on the Raspberry Pi.
