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

### Day 17
Reviewed the stack overflow trigger in the kernel entry point and confirmed that the recursion pattern is conceptually correct for demonstrating an overflow in a bare-metal OS. Identified that the project still needs a proper guard-page or exception-based stack overflow handling setup to behave like a real kernel-level stack overflow scenario. Continued improving understanding of bare-metal stack behavior, interrupt handling, and the operating system boot flow.

### Day 18
Fixed the GDT initialization issue in `src/gdt.rs` by correcting the `lazy_static!` block, modernizing the x86_64 API calls, and updating the TSS stack setup to the current Rust semantics. Verified that the project now builds and runs successfully with `cargo run`, confirming the kernel boot sequence proceeds without the earlier compile-time errors.

### Today’s Work
- Reviewed the Rust kernel code and identified the root cause behind the build failure.
- Fixed the GDT setup in `src/gdt.rs` using the correct `lazy_static!` pattern and current x86_64 API names.
- Verified the project compiles successfully with `cargo build`.
- Ran the OS in QEMU with `cargo run`, confirming the boot workflow proceeds without the original compile-time failure.
- Reviewed the stack overflow example and clarified the difference between a recursive overflow demonstration and a proper guard-page based kernel overflow handling strategy.
- Continued improving understanding of bare-metal Rust, interrupt handling, and kernel initialization flow.

## Notes

This project is a learning journey focused on operating system concepts, Rust `no_std` development, bootloader integration, and bare-metal testing.

## Future Goals

- improve kernel structure
- add more OS features
- expand VGA and serial debugging tools
- work toward a more complete minimal kernel
