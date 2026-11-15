# SRIKESH OS — System Architecture

## 1. System Overview

SRIKESH OS is a small x86-64 operating-system kernel written in Rust using no_std.

The project contains two principal layers:

- Host-side Cargo application: builds and launches virtual machine images.
- Freestanding kernel: executes inside the virtual machine.

## 2. Boot Architecture

The root build.rs obtains the compiled kernel artifact and uses the bootloader crate to generate BIOS and UEFI disk images.

The bootloader loads the kernel and transfers execution to kernel_main.

The kernel initializes serial output, the Interrupt Descriptor Table (IDT), the Programmable Interrupt Controller (PIC) and the Programmable Interval Timer (PIT).

## 3. CPU and Memory Information

The CPU module uses CPUID to report processor information.

The memory module reads the bootloader-provided memory map and reports usable RAM regions and kernel size.

## 4. Interrupt Architecture

The kernel uses an Interrupt Descriptor Table (IDT).

The timer interrupt is assigned interrupt vector 32.

The PIC is configured with offsets 0x20 and 0x28. The current configuration enables the master PIC timer IRQ while masking the other PIC IRQ lines.

The PIT is configured to generate approximately 100 interrupts per second.

### Timer Interrupt Flow

1. The PIT generates a timer event.
2. The PIC signals the CPU.
3. The CPU transfers control to the timer interrupt wrapper.
4. Assembly saves the general-purpose registers.
5. The saved frame pointer is passed to the Rust handler.
6. Rust increments the timer counter and acknowledges the PIC.
7. The scheduler decides whether the current task continues or another task should run.
8. Assembly restores the selected task context.
9. iretq resumes task execution.

## 5. Task Control Block (TCB)

Each Task Control Block stores:

- Task identifier.
- Task status.
- Time-slice configuration.
- Cooperative CPU context.
- Interrupt context stack pointer.
- Interrupt context stack.
- Cooperative execution stack.
- Task entry function.

The TCB uses PhantomPinned because the interrupt-driven scheduler stores raw pointers to permanently allocated TCBs.

The two demonstration TCBs are statically allocated.

## 6. Task States

| State | Meaning |
|---|---|
| Ready | Eligible for CPU execution |
| Running | Currently selected for execution |
| Blocked | Not currently eligible for execution |
| Terminated | Task has completed |

The demonstration exercises Ready and Running states in the preemptive scheduler. Blocked and Terminated are represented, but complete blocking, wake-up and termination mechanisms are not implemented.

## 7. Ready Queue

The ready queue is implemented as a circular buffer.

- Capacity: 8 task pointers.
- Head: next task to dequeue.
- Tail: next insertion position.
- Length: number of queued tasks.

Enqueue accepts non-null tasks in the Ready state. Dequeue removes the next available task.

The scheduler marks a dispatched task as Running.

## 8. Round Robin Scheduler

The demonstration starts two tasks.

Both tasks are inserted into the ready queue. The scheduler dispatches the first task.

The timer scheduler counts timer interrupts. After five ticks, it attempts to:

1. Save the interrupted task context.
2. Mark the current task Ready.
3. Enqueue the current task.
4. Dispatch the next Ready task.
5. Update the current-task pointer.
6. Return the next task saved interrupt stack pointer.

This produces timer-driven preemptive task execution.

## 9. Memory Allocator

The memory module contains a fixed-size 64 KB heap aligned to 4096 bytes.

The allocator:

- Rejects zero-sized allocation requests.
- Requires a nonzero power-of-two alignment.
- Uses checked arithmetic.
- Aligns allocation addresses.
- Uses an atomic compare-and-exchange loop to reserve heap space.
- Rejects requests exceeding heap capacity.

The allocator is a bump allocator. It does not support freeing memory or reusing earlier allocations.

It is a demonstration allocator and is not registered as the Rust global allocator.

## 10. Synchronization

The synchronization module uses spin::Mutex to protect a shared counter.

The demonstration increments the counter while holding the lock and verifies its value.

This demonstrates basic lock usage, not full contention behavior under concurrently scheduled tasks.

## 11. Context Switching

The timer interrupt wrapper saves general-purpose registers using assembly.

The Rust timer handler receives the saved context and asks the scheduler whether another task should execute.

The selected task context is restored by the assembly wrapper before returning with iretq.

The initial task launcher uses a synthetic interrupt stack frame. Its working layout was verified through BIOS QEMU execution.

## 12. Serial Diagnostics

The serial module provides output for:

- Kernel boot messages.
- CPU and memory information.
- Allocator test results.
- Scheduler messages.
- Task execution progress.

Serial output is the primary observation channel during headless QEMU testing.
