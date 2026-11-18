# Unsafe Rust Safety Review

## 1. Purpose

Rust provides memory-safety guarantees, but operating-system development requires controlled use of unsafe operations for hardware access, raw pointers and assembly.

## 2. Reviewed Areas

| Component | Unsafe operation | Required safety invariant |
|---|---|---|
| CPU context | Assembly context switching | Both stacks and saved register frames must be valid. |
| Task control | Pin and unchecked mutable access | Pinned task structures must remain at stable addresses. |
| Scheduler | Raw TCB pointer dereferencing | Pointers must reference valid initialized task structures. |
| Ready queue | Raw pointer validation | Enqueued pointers must remain valid while stored. |
| Interrupt handler | Assembly register restoration | Saved stack frames must match the assembly restore sequence. |
| PIT timer | Port I/O assembly | I/O instructions must execute in the intended kernel environment. |
| PIC | Controller operations | PIC must be initialized and acknowledged correctly. |
| Serial driver | Port I/O assembly | Serial hardware ports must be configured correctly. |
| CPU detection | CPUID intrinsic | The target CPU must support the requested instruction. |

## 3. Pinning

Task Control Blocks use Pin and PhantomPinned to prevent movement after their task stacks and execution contexts are initialized.

The unsafe Pin construction relies on the task structures having stable static storage.

## 4. Raw Pointer Management

The scheduler and ready queue use raw pointers to Task Control Blocks. Correct operation depends on pointer validity, task lifetime and controlled mutation.

## 5. Assembly and Interrupt Handling

The context-switching and interrupt routines depend on exact agreement between Rust data structures, stack layout, register order and x86-64 calling conventions.

These low-level invariants require careful review whenever assembly or task-frame structures change.

## 6. Hardware Access

PIT, PIC and serial drivers use port I/O instructions. These operations are restricted to the intended kernel execution environment.

## 7. Limitations

This review documents the intended safety assumptions. It is not a formal proof of memory safety or a substitute for exhaustive testing.

The kernel is an educational prototype and has not undergone formal verification or physical-hardware validation.
