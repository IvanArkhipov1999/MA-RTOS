# Rust example for risc-v esp32c6 architecture

Presented here is a straightforward Rust example utilizing Martos with a cooperative scheduler.

The example begins with a main task that handles a counter, incrementing it by 1 and printing the updated value.
An inner task is dynamically added once the counter reaches 25, and it increments the counter by 1. The inner task
periodically terminates itself when the counter is a multiple of 10, showcasing dynamic task creation and cooperative
task switching.

## How to install dependencies

For comprehensive guidance on installing the necessary dependencies for developing applications targeting the RISC-V
ESP32-C6 architecture,
please refer to [the official website](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html).
Below is an illustrative example demonstrating the installation of building toolchains on a Linux (Ubuntu/Debian):

```
apt-get -qq update
apt-get install -y -q build-essential curl
curl https://sh.rustup.rs -sSf | sh -s -- -y
cargo install espup
espup install
```

## How to build the example

For a thorough guide on developing projects for the RISC-V ESP32-C6 architecture across various operating systems,
we recommend
consulting [the official website](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html#3-set-up-the-environment-variables).
Below, you will find an illustrative example showcasing the building process on a Linux system (Ubuntu/Debian):

```
. $HOME/export-esp.sh
cargo build
```

## How to run the example

For detailed instructions on running projects for the RISC-V ESP32-C6 architecture across various operating systems,
we recommend consulting [the official website](https://docs.esp-rs.org/book/tooling/espflash.html).
Below, you will find an illustrative example showcasing the running on a Linux system (Ubuntu/Debian):

```
cargo run
```
