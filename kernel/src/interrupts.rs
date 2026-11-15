use core::arch::global_asm;
use core::sync::atomic::{AtomicU64, Ordering};

use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::VirtAddr;

pub const TIMER_INTERRUPT_ID: u8 = 32;

pub static TIMER_TICKS: AtomicU64 = AtomicU64::new(0);

unsafe extern "C" {
    fn srikesh_timer_interrupt_wrapper();
}

extern "C" fn timer_interrupt_rust(frame: usize) -> usize {
    TIMER_TICKS.fetch_add(1, Ordering::Relaxed);
    crate::pic::end_of_interrupt(TIMER_INTERRUPT_ID);
    crate::timer_schedule(frame)
}

global_asm!(
    ".global srikesh_timer_interrupt_wrapper",
    ".type srikesh_timer_interrupt_wrapper,@function",
    "srikesh_timer_interrupt_wrapper:",
    "push rax",
    "push rbx",
    "push rcx",
    "push rdx",
    "push rsi",
    "push rdi",
    "push rbp",
    "push r8",
    "push r9",
    "push r10",
    "push r11",
    "push r12",
    "push r13",
    "push r14",
    "push r15",
    "mov rdi, rsp",
    "and rsp, -16",
    "cld",
    "call {handler}",
    "mov rsp, rax",
    "pop r15",
    "pop r14",
    "pop r13",
    "pop r12",
    "pop r11",
    "pop r10",
    "pop r9",
    "pop r8",
    "pop rbp",
    "pop rdi",
    "pop rsi",
    "pop rdx",
    "pop rcx",
    "pop rbx",
    "pop rax",
    "iretq",
    ".size srikesh_timer_interrupt_wrapper, .-srikesh_timer_interrupt_wrapper",
    handler = sym timer_interrupt_rust,
);

static IDT: spin::Lazy<InterruptDescriptorTable> = spin::Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();

    idt.double_fault.set_handler_fn(double_fault_handler);

    unsafe {
        idt[TIMER_INTERRUPT_ID]
            .set_handler_addr(VirtAddr::new(
                srikesh_timer_interrupt_wrapper as *const () as u64,
            ));
    }

    idt
});

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn double_fault_handler(
    _stack_frame: x86_64::structures::idt::InterruptStackFrame,
    _error_code: u64,
) -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
