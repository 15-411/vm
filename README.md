# 15-411 VM for Abstract Assembly

This repo contains the source for the Abstract Assembly Virtual
Machine that can be used to test compilers created for the 15-411
Compilers course. For now, the VM will only guarantee to support C0
features up to Lab 3. The abstract assembly will respresent the SSA
form of a CFG produced in the intermediate stages of a compiler.


## TOV

- [Installation](#installation)
- [Using the VM](#usage)
- [Extending the VM](#extending-the-vm)
- [Abstract Assembly Structure](#abstract-assembly-structure)

## Installation
To actually build and run the VM, you will need to install the current stable version of Rust. We recommend that you use `rustup`, which is a Rust toolchain and version manager. It can download, install, manage, and update your Rust tools on your machine easily.

To download and install `rustup`, follow the instructions on this page: [https://rustup.rs/](https://rustup.rs/). This step will also download the latest Rust compiler and `cargo`, the Rust Project Manager. Instructions on building and running the project using `cargo` can be found in [Using the VM](#usage).


## Usage
Run the following to use the VM:
```rust
cargo run --bin main -- <path-to-abs-file>
```

If you want the VM to run over multiple test cases, you should use the builtin VMRunner instead. This is essentially a copy of the `gradecompiler` tool for the VM. Run the following to access the VMRunner:
```rust
cargo run --bin runner -- <path-to-test-case-dir>
```

If you want to build the VM and VMRunner as a portable executable, run the following. Note that the release option will build an optimized version of the two tools.
```rust
cargo build [--release]
```


## Extending the VM
Come back for more information about extending the VM to include you're own instructions and functionality.

## Abstract Assembly Structure
For more information about the structure of the abstract assembly language, please take a look at the `format.md` Markdown file located in the VM repo.
