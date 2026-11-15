# SRIKESH OS — Debugging and Validation Log

## 1. Clean Build
The extracted project completed a successful release build.
Non-fatal compiler warnings remained.

## 2. BIOS Boot
QEMU BIOS testing verified kernel initialization, timer operation,
allocator tests, synchronization, scheduler operation and task execution.

## 3. UEFI Boot
QEMU UEFI testing verified the same runtime demonstrations.

## 4. Scheduler Validation
Eight scheduler self-tests passed, covering queue handling,
task validation, FIFO dispatch, time-slice expiration and queue capacity.

## 5. Memory Validation
Five allocator tests passed, covering invalid requests, alignment,
separate allocations, heap accounting and heap exhaustion.

## 6. Synchronization Validation
The shared counter completed 1000 protected updates using a spinlock
in both BIOS and UEFI runtime tests.

## 7. Performance Instrumentation
Runtime counters recorded 84 context switches,
43 Task 1 dispatches and 42 Task 2 dispatches
in both tested boot modes.

## 8. QEMU Timeout
The 15-second QEMU test was intentionally stopped using timeout.
Exit status 124 is expected and does not itself indicate a kernel crash.

## 9. Documentation and Graph Validation
Architecture diagrams and performance graphs were generated.
PNG, SVG and Python script validation completed successfully.

## 10. Known Limitations
No physical-hardware or long-duration testing is claimed.
The synchronization demonstration is not a multi-core stress test.
Memory deallocation and complete blocked-task wake-up are not implemented.
