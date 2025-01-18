# Martos
![Martos ci workflow](https://github.com/IvanArkhipov1999/Martos/actions/workflows/rust.yml/badge.svg)
[![Version](https://img.shields.io/crates/v/martos.svg)](https://crates.io/crates/martos)
[![Release](https://img.shields.io/github/v/release/IvanArkhipov1999/Martos)](https://github.com/IvanArkhipov1999/Martos/releases)
[![codecov](https://codecov.io/gh/Azernem/Martos/branch/main/graph/badge.svg?token=SLIHSUWHT2)](https://codecov.io/gh/Azernem/Martos)

Martos is an elegant real-time operating system designed for creating complex multi-agent systems. 
Developers have the flexibility to write software for Martos using either Rust (preferred) or C languages.

Martos supports following features:
- non-preemptive task manager based on Round Robin algorithm;
- memory allocation;
- interaction with hardware timers.

## Programming in Rust
To develop software in Rust, you have the option to incorporate the Martos as a dependency:
```
[dependencies]
martos = "0.4.0"
```

You can explore a variety of Rust examples showcasing different architectures in the ['examples/rust-examples'](https://github.com/IvanArkhipov1999/Martos/tree/main/examples/rust-examples) directory.

## Programming in C
To develop software in C, you have to link the Martos static library with your project:
```
-Lpath-to-static-lib -lxtensa_esp32_static_lib
```

You can obtain the Martos static library for supported architectures from either the release artifacts or the continuous integration (CI) artifacts.
If you wish to compile the Martos static library on your own, please refer to the ['c-library'](https://github.com/IvanArkhipov1999/Martos/tree/main/c-library) directory, 
which includes static library targets tailored for various architectures.

You can explore diverse C examples tailored for various architectures in the ['examples/c-examples'](https://github.com/IvanArkhipov1999/Martos/tree/main/examples/c-examples) directory.
