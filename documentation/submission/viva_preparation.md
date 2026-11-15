# Micro-OS Engineering Project — Viva Preparation

## 1. Project Overview

**Q1. What is the objective of this project?**

To design and implement a small operating-system kernel using Rust and demonstrate fundamental OS mechanisms.

**Q2. Why Rust?**

Rust provides ownership, borrowing and memory-safety features useful for low-level systems programming.

**Q3. What is a kernel?**

The kernel is the core of an operating system. It manages system resources and provides essential services.

**Q4. BIOS vs UEFI?**

BIOS is a traditional firmware boot interface. UEFI is a modern firmware interface. This project was tested using both.

## 2. Scheduling

**Q5. What is CPU scheduling?**

It is the mechanism that decides which task gets CPU execution time.

**Q6. What scheduling specialization did you implement?**

A configurable scheduling quantum of 5 timer ticks.

**Q7. What is a time quantum?**

The time slice allocated to a task before the scheduler considers switching tasks.

**Q8. What is a context switch?**

The change from executing one task to executing another.

**Q9. How did you measure scheduler activity?**

Using atomic counters for context switches and Task 1 and Task 2 dispatches.

**Q10. How many scheduler tests were implemented?**

Eight tests covering queue conditions, invalid tasks, FIFO dispatch, time-slice expiration and queue capacity.

## 3. Memory Management

**Q11. What is memory allocation?**

The process of reserving memory for a program or kernel operation.

**Q12. What is memory alignment?**

Ensuring that memory addresses satisfy the alignment requirements of the data.

**Q13. How many allocator tests passed?**

Five tests covering invalid requests, alignment, separate allocations, heap accounting and heap exhaustion.

## 4. Synchronization

**Q14. What is synchronization?**

Coordinating access to shared resources to prevent conflicting operations.

**Q15. What is a mutex?**

A mutual-exclusion mechanism that protects a shared resource from simultaneous access.

**Q16. What synchronization test did you perform?**

A spin mutex protected a shared counter through 1000 updates. This was not a multi-core stress test.

## 5. Testing and Performance

**Q17. How did you test the kernel?**

By building and running BIOS and UEFI configurations in an emulator and observing runtime test results.

**Q18. What performance results were recorded?**

Both observed runs recorded:
- 84 context switches
- 43 Task 1 dispatches
- 42 Task 2 dispatches
- 5 timer ticks per quantum

**Q19. What is the approximate time quantum in milliseconds?**

At approximately 100 PIT ticks per second, 5 ticks correspond to approximately 50 milliseconds.

**Q20. Were BIOS and UEFI results different?**

The recorded scheduler counters were the same in the two observed runs. This does not prove identical performance under all workloads.

**Q21. Why use Python graphs?**

To visualize the collected scheduler measurements and make comparisons easier.

## 6. Limitations and Future Work

**Q22. What are the limitations?**

This is an educational micro-OS prototype. Its functionality and testing scope are limited compared with a production operating system.

**Q23. What improvements are possible?**

Broader workload testing, additional scheduling policies and more extensive synchronization and memory-management tests.

**Q24. What did you learn?**

Practical Rust systems programming, kernel structure, scheduling, memory management, synchronization, emulator testing and performance documentation.

## 7. Verified Project Values

| Measurement | Result |
|---|---:|
| Scheduler quantum | 5 timer ticks |
| Approximate PIT frequency | 100 Hz |
| Approximate time slice | 50 ms |
| Context switches | 84 |
| Task 1 dispatches | 43 |
| Task 2 dispatches | 42 |
| Scheduler tests | 8 |
| Allocator tests | 5 |
| Protected synchronization updates | 1000 |
| Boot modes tested | BIOS and UEFI |

## 8. Viva Advice

- Explain concepts in your own words.
- Be ready to show the relevant Rust source files.
- Explain how scheduler counters are collected.
- Distinguish measured results from theoretical expectations.
- Be honest about limitations.
- Do not claim features or tests that were not implemented.
