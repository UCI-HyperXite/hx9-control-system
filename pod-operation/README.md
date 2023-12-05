# Pod Operation

This Rust package is for the main program to be run on the pod.
The program runs a finite-state machine to operate the pod components
and acts as a Socket.IO server to communicate with the control station.

## Usage

### First-time Setup

Install cargo-watch

```shell
cargo install cargo-watch
```

### Local Development

To locally run the development server with auto-reload

```shell
cargo watch -x run
```
