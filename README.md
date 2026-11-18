# SRIKESH OS — Rust Micro Operating System

## Overview

SRIKESH OS is an educational x86-64 micro-operating-system project developed in Rust.
It demonstrates hardware initialization, interrupt handling, task management,
scheduling, synchronization and memory allocation.

The kernel uses no_std Rust and the bootloader API.

## Features

- BIOS and UEFI boot image generation.
- CPU information using CPUID.
- Boot-time memory information reporting.
- Serial diagnostics.
- Framebuffer welcome screen.
- Interrupt Descriptor Table (IDT).
- Programmable Interrupt Controller (PIC8259).
- Programmable Interval Timer (PIT) configured at 100 Hz.
- Timer interrupt handling.
- Task Control Blocks and task states.
- Circular ready queue with capacity eight.
- Preemptive Round Robin scheduling.
- Assembly-assisted interrupt context switching.
- Fixed-size 64 KB atomic bump allocator.
- Spinlock synchronization demonstration.

## Technology Stack

- Rust nightly toolchain
- x86-64 architecture
- bootloader_api and bootloader
- QEMU emulator
- pic8259 interrupt controller library
- x86_64 Rust library
- spin synchronization library
- font8x8 font library

## Requirements

- Ubuntu or another Linux environment.
- Rust nightly toolchain.
- x86_64-unknown-none compilation target.
- LLVM tools.
- QEMU x86-64 emulator.

Install the required Rust components if missing:

```bash
rustup toolchain install nightly
rustup component add llvm-tools --toolchain nightly
rustup target add x86_64-unknown-none --toolchain nightly
```

Install QEMU on Ubuntu if required:

```bash
sudo apt update
sudo apt install qemu-system-x86
```

## Building the Project

Open the project root directory and run:

```bash
cargo build --release
```

The build script generates BIOS and UEFI disk images.

## Running the Operating System

To launch using BIOS:

```bash
cargo run --release -- bios
```

To launch using UEFI:

```bash
cargo run --release -- uefi
```

The launcher uses QEMU. UEFI mode also requires OVMF firmware.

## Runtime Verification

BIOS boot was tested using QEMU.

The following demonstrations were observed:

1. Kernel boot and initialization.
2. CPU and memory information reporting.
3. PIT timer test.
4. Spinlock synchronization demonstration.
5. Memory allocator validation and exhaustion tests.
6. Initial dispatch of two tasks.
7. Both tasks starting and executing.
8. Alternating task execution through timer-driven Round Robin scheduling.

A separate 15-second QEMU run confirmed that both tasks continued producing progress messages without an observed crash.

## Limitations

- The bump allocator does not support deallocation or memory reuse.
- It is not registered as Rust global allocator.
- Synchronization is demonstrated using a protected shared counter.
- The scheduler currently demonstrates two statically allocated tasks.
- Blocked and terminated states exist, but complete blocking, wake-up and termination mechanisms are not implemented.
- BIOS and UEFI runtime behavior have been demonstrated in QEMU. UEFI evidence includes kernel initialization and timer-driven execution of both tasks.

## Future Improvements

- Dynamic task creation and destruction.
- Blocking and wake-up queues.
- Memory deallocation.
- Global allocator integration.
- Stronger synchronization tests.
- Broader UEFI testing across repeated runs and additional workloads.
- Keyboard and additional device drivers.

## Project Goal

This project demonstrates how a small Rust kernel initializes hardware, handles interrupts, manages task contexts and schedules multiple tasks on an x86-64 processor.
