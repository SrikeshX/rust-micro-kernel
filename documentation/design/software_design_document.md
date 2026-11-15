# SRIKESH OS — Software Design Document

## 1. Introduction

SRIKESH OS is an experimental x86-64 micro operating system developed using Rust. It demonstrates kernel initialization, interrupt handling, task scheduling, context switching, memory allocation and synchronization.

## 2. Design Objectives

- Implement a small Rust-based no_std kernel.
- Demonstrate BIOS and UEFI boot support.
- Handle hardware timer interrupts.
- Implement a Round Robin task scheduler.
- Demonstrate task context switching.
- Provide basic memory allocation and synchronization.
- Validate functionality through QEMU runtime tests.

## 3. System Architecture

The project consists of a host-side Cargo application, bootloader integration and a freestanding kernel.

The bootloader loads the kernel and transfers execution to kernel_main. The kernel initializes serial diagnostics, CPU information, memory information, interrupt handling, the programmable interrupt controller and the programmable interval timer.

The timer interrupt drives scheduler decisions and task context switching.

## 4. Kernel Components

### 4.1 CPU and Memory Information
The CPU module obtains processor information. The memory module manages a fixed-size kernel heap using a bump allocation strategy.

### 4.2 Interrupt Handling
The interrupt subsystem configures the Interrupt Descriptor Table and handles timer interrupts. The PIC manages hardware interrupt delivery, while the PIT provides periodic timer events.

### 4.3 Task Control Block
Each task is represented by a Task Control Block containing its identifier, state, scheduling information and context-related data.

### 4.4 Ready Queue
The ready queue uses a circular-buffer design with a capacity of eight task entries. It supports FIFO dispatch and validates task pointers and states.

### 4.5 Scheduler
The scheduler uses the Round Robin algorithm. Its configured time quantum is five PIT ticks.

### 4.6 Context Switching
The context-switching mechanism saves the current task context and restores the selected task context using Rust and assembly routines.

### 4.7 Synchronization
A spinlock-based mutex protects a shared counter during the synchronization demonstration.

### 4.8 Serial Diagnostics
The serial module provides diagnostic output for kernel initialization, tests and runtime performance information.

## 5. Task Scheduling Strategy

Two statically initialized tasks demonstrate Round Robin scheduling.

The timer interrupt tracks elapsed ticks. When the configured time slice expires, the scheduler selects another ready task and performs a context switch.

The configured PIT frequency is approximately 100 Hz. A five-tick quantum therefore corresponds to approximately 50 milliseconds.

## 6. Memory Management Strategy

The kernel uses a fixed-size 64 KB heap with a bump allocator. Allocation checks cover invalid requests, alignment, separate allocations, heap accounting and exhaustion.

Memory deallocation and global allocator integration are not implemented.

## 7. Testing Strategy

The operating system is validated through release builds and QEMU runtime testing.

Testing covers:
- BIOS and UEFI boot execution.
- PIT timer operation.
- Scheduler queue and dispatch behavior.
- Memory allocator operations.
- Spinlock-protected counter updates.
- Task execution and context switching.

Detailed results are available in TESTING.md and documentation/testing/test_results.md.

## 8. Design Constraints

The current implementation is an educational prototype.

Its limitations include:
- Two statically initialized demonstration tasks.
- Fixed-size ready queue.
- Fixed-size bump allocator.
- No memory deallocation.
- No complete blocked-task wake-up implementation.
- No physical-hardware validation.

## 9. Future Improvements

Potential future improvements include dynamic task management, memory deallocation, improved synchronization, additional scheduling policies and physical-hardware testing.

## 10. Conclusion

SRIKESH OS demonstrates fundamental operating-system mechanisms using Rust, including boot initialization, interrupt handling, Round Robin scheduling, context switching, memory allocation and synchronization.

BIOS and UEFI QEMU tests provide runtime evidence of the implemented functionality.
