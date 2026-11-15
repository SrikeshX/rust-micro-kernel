//! CPU register state saved when a timer interrupt occurs.
//!
//! The field order is designed for a future assembly interrupt wrapper.
//! The structure does not yet perform context switching.

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct InterruptContext {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rbp: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rax: u64,

    // Same-privilege x86-64 interrupt return frame.
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
}

pub const INTERRUPT_CONTEXT_SIZE: usize =
    core::mem::size_of::<InterruptContext>();

const _: () = assert!(INTERRUPT_CONTEXT_SIZE == 18 * 8);

// Compile-time verification of the assembly/Rust memory layout.
const _: () = {
    use core::mem::offset_of;

    assert!(offset_of!(InterruptContext, r15) == 0);
    assert!(offset_of!(InterruptContext, r14) == 8);
    assert!(offset_of!(InterruptContext, r13) == 16);
    assert!(offset_of!(InterruptContext, r12) == 24);
    assert!(offset_of!(InterruptContext, r11) == 32);
    assert!(offset_of!(InterruptContext, r10) == 40);
    assert!(offset_of!(InterruptContext, r9) == 48);
    assert!(offset_of!(InterruptContext, r8) == 56);
    assert!(offset_of!(InterruptContext, rbp) == 64);
    assert!(offset_of!(InterruptContext, rdi) == 72);
    assert!(offset_of!(InterruptContext, rsi) == 80);
    assert!(offset_of!(InterruptContext, rdx) == 88);
    assert!(offset_of!(InterruptContext, rcx) == 96);
    assert!(offset_of!(InterruptContext, rbx) == 104);
    assert!(offset_of!(InterruptContext, rax) == 112);
    assert!(offset_of!(InterruptContext, rip) == 120);
    assert!(offset_of!(InterruptContext, cs) == 128);
    assert!(offset_of!(InterruptContext, rflags) == 136);
};
