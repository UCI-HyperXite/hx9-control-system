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

To compile for the Raspberry Pi target, a specific linker is needed for each operating system,
or [`cross`](https://github.com/cross-rs/cross) can be used to build inside a container.

#### macOS

A Homebrew formula for macOS cross-compiler toolchains is available

```shell
brew tap messense/macos-cross-toolchains
brew install armv7-unknown-linux-gnueabihf
```

#### Windows/Linux

To cross-compile on Windows and Linux, a different compiler toolchain is needed. From the
[Arm GNU Toolchain Downloads](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads),
download and install the **AArch32 GNU/Linux target with hard float (arm-none-linux-gnueabihf)**
for your operating system.

#### Alternative Building Process With `cross`

An alternative to installing cross-compilers is using `cross` to build and run the Rust project
using containers and emulation. This can be used on any operating system (macOS, Windows, Linux).

**Prerequisites**

First, install [Docker](https://docs.docker.com/). You can either install
Docker Desktop or the more lightweight Docker Engine, which is only available
on Linux distributions.

**Please note that on Linux, non-sudo users need to be in the `docker` group or
use rootless Docker.** You can read more about adding yourself to the group
[here](https://docs.docker.com/engine/install/linux-postinstall/#manage-docker-as-a-non-root-user).

Next, install the `cross` package.

```shell
cargo install cross
```

This is the library that will facilitate cross-compilation with minimal additional configuration.
`cross` requires Docker (or Podman) in order to function and will also emulate the ARM architecture
inside the container using QEMU.

On Linux, if an error appears during build about `GLIBC`, install the `glibc` library.

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

Uncomment the arm-linux linker for your operating system in `.cargo/config.toml`.

To build for production, use the `--release` option:

```shell
cargo build --target armv7-unknown-linux-gnueabihf --release
```

Alternatively, use `cross` to compile in a container:

```shell
cross build --release
```

Note: the default target is already specified in `Cross.toml`.

Either approach will compile the project to
`target/armv7-unknown-linux-gnueabihf/release/pod-operation`
which can be run on the Raspberry Pi.
