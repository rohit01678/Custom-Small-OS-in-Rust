# Custom Small OS in Rust

A personal project inspired by blog OS development, built in Rust to learn how an operating system boots, renders text to a VGA buffer, runs basic tests, and interacts with QEMU.

## Overview

This project focuses on creating a minimal operating system kernel in Rust using a custom bootloader setup. It includes:

- a minimal Rust kernel entry point
- VGA text output support
- serial output support
- custom test infrastructure for no_std environments
- QEMU-based execution and testing

## Prerequisites

Before running this project, install the following:

1. Rust and Cargo
2. Rust nightly toolchain
3. QEMU virtual machine
4. The required nightly target and bootloader dependencies for the project

## Setup

Make sure you are using the nightly toolchain required by the project, since this OS project uses features that are not available in stable Rust.

## Commands

```bash
cargo run
cargo test
```

- `cargo run` runs the OS in QEMU.
- `cargo test` runs the custom Rust kernel tests.

## Project Progress

### Day 1 & 2
Learned the basics of Rust, including printing text and setting up a basic project structure for version control and showcase.

### Day 3
Created the JSON configuration needed for enabling and disabling important functions required for the project setup.

### Day 4
Started adding code to produce text in a virtual machine using VGA output and QEMU.

### Day 5
Added the `vga_buffer.rs` file for printing text on the bootloader screen. The implementation was still evolving and required more work.

### Day 6
Deepened understanding of Rust concepts and kernel development fundamentals, with more focus on debugging and project structure.

### Day 7
Fixed the dependency and command setup so the project can run through `cargo run` more cleanly.

### Day 8 & 9
Completed the VGA text buffer implementation and added print/println support for outputting messages to the virtual machine screen.

### Day 10
Made modifications for kernel testing, since the standard library cannot be used in a bare-metal kernel environment.

### Day 11
Worked through several build issues and fixed problems caused by project changes, restoring successful execution.

### Day 12
Edited and improved the VGA buffer code and created test cases to validate the functionality.

### Day 13
Added a new boot test file in the `tests` folder to implement a custom boot testing framework.

### Day 14
Extended the kernel test infrastructure in `lib.rs` and `basic_boot.rs` to improve integrity and future testing support.

### Day 15
Completed testing of the project’s custom test setup and resolved the panic-related integration test issue, confirming the OS tests now pass successfully.

### Day 16
Fixed the compile issue in the interrupt descriptor table setup by correcting the `lazy_static!` pattern and aligning the exported initialization function with the kernel entry point. Confirmed that the project builds successfully again after the fix.

### Today’s Work
- Reviewed the Rust kernel code and identified the root cause behind the build failure.
- Fixed the interrupt initialization bug in `src/interrupts.rs`.
- Updated the library init function in `src/lib.rs` to match the entry-point call pattern.
- Verified the project builds cleanly using `cargo build --quiet`.
- Continued improving understanding of bare-metal Rust, interrupt handling, and kernel initialization flow.

## Notes

This project is a learning journey focused on operating system concepts, Rust `no_std` development, bootloader integration, and bare-metal testing.

## Future Goals

- improve kernel structure
- add more OS features
- expand VGA and serial debugging tools
- work toward a more complete minimal kernel
