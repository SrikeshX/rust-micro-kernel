# SRIKESH OS — Test Results

## Test Environment
- Architecture: x86-64
- Kernel: Rust no_std
- Emulator: QEMU
- Boot modes: BIOS and UEFI

## Runtime Test Results

| Test | BIOS | UEFI |
|---|---|---|
| Kernel boot | Passed | Passed |
| PIT timer: 100 ticks | Passed | Passed |
| Scheduler self-tests: 8 | Passed | Passed |
| Allocator tests: 5 | Passed | Passed |
| Synchronization: 1000 updates | Passed | Passed |
| Task 1 and Task 2 execution | Observed | Observed |
| Context switching | Observed | Observed |

## Scheduler Performance

| Metric | BIOS | UEFI |
|---|---:|---:|
| Context switches | 84 | 84 |
| Task 1 dispatches | 43 | 43 |
| Task 2 dispatches | 42 | 42 |
| Quantum | 5 ticks | 5 ticks |
| PIT frequency | 100 Hz | 100 Hz |
| Approximate time slice | 50 ms | 50 ms |

Raw measurements: ../performance/scheduler_results.csv

## Limitations
- Measurements are runtime snapshots, not long-duration benchmarks.
- No physical-hardware testing is claimed.
- No multi-core synchronization stress test is claimed.
- Memory deallocation is not implemented.
