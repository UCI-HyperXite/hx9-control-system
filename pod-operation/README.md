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
rustup target add aarch64-unknown-linux-gnu
```

### Cross-Compilation

To compile for the Raspberry Pi target, a specific linker is needed for each operating system,
or [`cross`](https://github.com/cross-rs/cross) can be used to build inside a container.

#### macOS

A Homebrew formula for macOS cross-compiler toolchains is available

```shell
brew tap messense/macos-cross-toolchains
brew install aarch64-unknown-linux-gnu
```

#### Windows/Linux

To cross-compile on Windows and Linux, a different compiler toolchain is needed. From the
[Arm GNU Toolchain Downloads](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads),
download and install the **AArch64 GNU/Linux target (aarch64-none-linux-gnu)** for your operating
system.

For Linux operating systems, your distribution may also provide the target via
its package manager (`apt`, `pacman`, etc.). Please refer to the appropriate
package repository to verify and install it this way if you wish.

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

Uncomment the aarch64 Linux linker for your operating system in `.cargo/config.toml`.

To do a release build for the Raspberry Pi, use the `brpi` alias:

```shell
cargo brpi
```

Alternatively, use `cross` to compile in a container:

```shell
cross build --release
```

Note: the default target is already specified in `Cross.toml`.

Either approach will compile the project to
`target/aarch64-unknown-linux-gnu/release/pod-operation`
which can be run on the Raspberry Pi.
