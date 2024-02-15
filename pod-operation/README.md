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

#### Windows

TBD

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
