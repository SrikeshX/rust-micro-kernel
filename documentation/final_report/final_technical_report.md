# SRIKESH OS
# Rust-Based Micro Operating System
## Final Technical Report

**Project:** Micro-OS Engineering Project  
**Author:** SRIKESH S  
**USN:** 25BTRCL258  
**Branch:** CSE AI & ML  

---

## Abstract

SRIKESH OS is an experimental x86-64 micro operating system developed using Rust. The project demonstrates fundamental operating-system mechanisms, including kernel initialization, hardware timer interrupts, task scheduling, context switching, memory allocation and synchronization.

The kernel uses a Round Robin scheduling strategy with two statically initialized demonstration tasks. A fixed-size ready queue manages task dispatch, while a bump allocator provides basic heap allocation. A spinlock-based mutex demonstrates protected access to a shared resource.

The system was built and tested using BIOS and UEFI boot modes in the QEMU emulator. Runtime tests verified timer operation, scheduler behavior, allocator operations, synchronization and task execution. Performance counters were also collected to observe scheduler activity.

## 1. Introduction

An operating system manages computer hardware and provides essential services for software execution. Its responsibilities include processor management, memory management, interrupt handling and task scheduling.

This project focuses on implementing and demonstrating selected operating-system mechanisms using Rust. Rust provides low-level control while offering memory-safety features that are useful in systems programming.

SRIKESH OS is an educational prototype intended to demonstrate how important kernel components interact.

## 2. Problem Statement

Understanding operating-system concepts requires more than theoretical study. Mechanisms such as interrupt handling, scheduling, context switching and memory allocation are easier to understand when implemented and observed in a working environment.

The project addresses this learning objective by developing a small Rust-based kernel that demonstrates these mechanisms and validates their behavior through emulator-based testing.

## 3. Project Objectives

- Develop an experimental x86-64 kernel using Rust and no_std.
- Support BIOS and UEFI boot execution.
- Initialize CPU, memory and interrupt-related components.
- Configure periodic timer interrupts.
- Implement a Round Robin task scheduler.
- Demonstrate task context switching.
- Implement basic heap allocation.
- Demonstrate spinlock-based synchronization.
- Validate kernel behavior using QEMU.
- Collect and document scheduler performance observations.

## 4. Technology Stack

The project uses the following technologies:

- Rust programming language for kernel implementation.
- no_std environment for freestanding kernel development.
- Cargo for project building.
- BIOS and UEFI bootloader support.
- x86-64 architecture.
- QEMU for virtual machine testing.
- Python and Matplotlib for performance graph generation.

## 5. System Architecture

The system consists of a host-side Cargo application, bootloader integration and a freestanding kernel.

The bootloader loads the kernel and transfers control to the kernel entry point. Kernel initialization configures serial diagnostics, CPU information, memory information and interrupt handling.

The Programmable Interrupt Controller manages hardware interrupts, while the Programmable Interval Timer generates periodic timer events.

The timer interrupt invokes scheduler logic, which manages task selection and context switching.

Refer to the architecture diagram:
- documentation/architecture/micro_os_architecture.png
- documentation/architecture/micro_os_architecture.svg

## 6. Kernel Implementation

### 6.1 Kernel Initialization

The kernel initializes its essential subsystems and reports initialization status through serial diagnostics.

### 6.2 Interrupt Management

The Interrupt Descriptor Table is configured to handle interrupts. The timer interrupt is used to drive scheduling activity.

### 6.3 Task Control Block

Each demonstration task has a Task Control Block containing task identification, scheduling state and context-related information.

### 6.4 Ready Queue

A circular ready queue with a capacity of eight entries manages task dispatch. It validates task pointers and task states and follows FIFO ordering.

### 6.5 Round Robin Scheduler

The scheduler uses two statically initialized demonstration tasks. A configurable quantum of five PIT ticks determines when scheduling decisions occur.

### 6.6 Context Switching

Assembly and Rust routines cooperate to save the current execution context and restore the selected task context.

### 6.7 Memory Management

A fixed-size 64 KB heap is managed using a bump allocator. Runtime tests check allocation validity, alignment, separate allocations, heap accounting and exhaustion.

### 6.8 Synchronization

A spinlock-based mutex protects a shared counter. The demonstration performs 1,000 protected updates and checks the final counter value.

## 7. Testing Methodology

The project was validated using release builds and QEMU virtual machine execution.

Both BIOS and UEFI boot modes were tested. Runtime checks covered kernel initialization, timer operation, scheduler behavior, memory allocation, synchronization and task execution.

### 7.1 Scheduler Tests

Eight scheduler self-tests were observed passing:

1. Empty queue handling.
2. Null task rejection.
3. Invalid task state rejection.
4. FIFO first dispatch.
5. Time-slice expiration.
6. FIFO second dispatch.
7. Null time-slice task handling.
8. Queue capacity validation.

### 7.2 Memory Allocator Tests

Five allocator tests were observed passing:

1. Invalid allocation request rejection.
2. Memory alignment.
3. Separate allocations.
4. Heap accounting.
5. Heap exhaustion.

### 7.3 Synchronization Test

The synchronization demonstration performed 1,000 protected updates to a shared counter using a spinlock. The expected final counter value was observed in both BIOS and UEFI tests.

### 7.4 Task Execution

Both demonstration tasks started and produced progress messages. Scheduler context switching was observed in both boot modes.

## 8. Performance Evaluation

Runtime counters were used to record scheduler activity.

| Metric | BIOS | UEFI |
|---|---:|---:|
| Context switches | 84 | 84 |
| Task 1 dispatches | 43 | 43 |
| Task 2 dispatches | 42 | 42 |
| Scheduler quantum | 5 ticks | 5 ticks |
| PIT frequency | 100 Hz | 100 Hz |
| Approximate time slice | 50 ms | 50 ms |

The recorded results show matching scheduler activity in the tested BIOS and UEFI runs.

These values are runtime snapshots triggered by Task 1 reaching a progress threshold. They are not fixed-duration benchmark measurements.

### 8.1 Performance Graphs

The following graphs were generated using Python and Matplotlib:

- documentation/performance/scheduler_activity_comparison.png
- documentation/performance/scheduler_quantum_analysis.png

Raw measurements are available in documentation/performance/scheduler_results.csv.

## 9. Limitations

The current project is an educational prototype. Its limitations include:

- Only two statically initialized demonstration tasks.
- Fixed-size ready queue.
- Fixed-size bump allocator.
- No memory deallocation.
- No Rust global allocator integration.
- No complete blocked-task wake-up implementation.
- No heavy multi-task or multi-core synchronization stress testing.
- No long-duration stability or physical-hardware testing.

## 10. Future Improvements

Future development could include:

- Dynamic task creation and termination.
- Complete blocked-task and wake-up management.
- Memory deallocation and improved heap management.
- Additional scheduling algorithms.
- More extensive synchronization testing.
- Long-duration and physical-hardware validation.

## 11. Conclusion

SRIKESH OS demonstrates fundamental operating-system mechanisms through a Rust-based x86-64 kernel.

The project implements boot initialization, timer interrupts, Round Robin scheduling, task context switching, basic memory allocation and spinlock-based synchronization.

BIOS and UEFI QEMU testing provided runtime evidence for the implemented demonstrations. Scheduler performance snapshots and Python-generated graphs provide additional evidence for evaluating scheduler activity.

The project provides a practical foundation for understanding low-level operating-system design and kernel development.

## 12. References

1. Rust Programming Language Documentation — https://doc.rust-lang.org/
2. Rust Embedded Book — https://docs.rust-embedded.org/book/
3. QEMU Documentation — https://www.qemu.org/docs/master/
4. Operating System Concepts — Abraham Silberschatz, Peter B. Galvin and Greg Gagne.
5. Project assignment reference — Operating System Concepts repository, CA6 Rust Micro OS Engineering Project.
