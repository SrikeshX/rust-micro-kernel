# SRIKESH OS — Testing and Verification

## 1. Build Verification

From the project root, execute:

```bash
cargo build --release
```

A successful build must finish without compilation errors. Compiler warnings may remain.

## 2. BIOS QEMU Testing

The BIOS image can be tested using QEMU.

Find the generated BIOS image:

```bash
BIOS=$(find target/release/build/micro-os -path "*/out/bios.img" -printf "%T@ %p\n" | sort -nr | head -1 | cut -d" " -f2-)
```

Run the operating system:

```bash
timeout 15s qemu-system-x86_64 -drive "format=raw,file=$BIOS" -m 256M -serial stdio -display none -no-reboot -no-shutdown
```

## 3. Expected Runtime Messages

The following messages indicate successful execution of the corresponding demonstrations:

```text
[INFO] PIT timer test passed: received 100 ticks
[INFO] Synchronization test passed
[INFO] All memory allocator tests passed
[PREEMPT] TASK 1 STARTED
[PREEMPT] TASK 2 STARTED
```

Both tasks should continue producing progress messages:

```text
[PREEMPT] TASK 1 RUNNING
[PREEMPT] TASK 2 RUNNING
```

## 4. Stability Test Results

A 15-second BIOS QEMU test was performed.

| Check | Observed result |
|---|---|
| Task 1 start message | 1 |
| Task 2 start message | 1 |
| Task 1 progress messages | 3 |
| Task 2 progress messages | 3 |
| QEMU crash | None observed |
| Emulator exit | Timeout after 15 seconds |

The timeout exit status was 124 because the test intentionally stopped the emulator. It does not indicate a kernel failure.

## 5. Memory Allocator Tests

The kernel performs the following runtime checks:

1. Reject invalid allocation requests.
2. Verify 16-byte and 64-byte alignment.
3. Verify that separate allocations do not overlap.
4. Verify heap usage and remaining-capacity accounting.
5. Consume the remaining heap and reject an additional allocation.

The exhaustion test intentionally consumes the remaining heap and runs once during the demonstration boot.

## 6. Synchronization Test

The kernel increments a shared counter under a spinlock and checks that the counter reaches the expected value.

This is a basic synchronization demonstration.

## 7. Test Scope and Limitations

The verified runtime evidence covers BIOS boot in QEMU.

The following are not claimed as fully verified:

- UEFI runtime execution.
- Dynamic task creation and destruction.
- Complete blocked-task wake-up behavior.
- Memory deallocation.
- Rust global allocator integration.
- Heavy multi-task lock contention.
- Long-duration or physical-hardware testing.

## 8. Detailed Test Documentation

See the following supporting records:

- `documentation/testing/test_results.md`
- `documentation/testing/debugging_log.md`
- `documentation/performance/scheduler_results.csv`
