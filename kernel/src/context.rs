use core::arch::global_asm;

pub const TASK_STACK_SIZE: usize = 4096;

#[repr(C)]
#[derive(Debug, Default)]
pub struct CpuContext {
    pub rsp: usize,
}

#[repr(align(16))]
pub struct TaskStack {
    data: [u8; TASK_STACK_SIZE],
}

impl TaskStack {
    pub const fn new() -> Self {
        Self {
            data: [0; TASK_STACK_SIZE],
        }
    }

    // Return the 16-byte-aligned top address of this stack.
    pub fn top(&mut self) -> usize {
        let end = self.data.as_mut_ptr() as usize + TASK_STACK_SIZE;
        end & !0xF
    }
}

global_asm!(
    ".global srikesh_context_switch",
    ".type srikesh_context_switch,@function",

    "srikesh_context_switch:",
    "push rbx",
    "push rbp",
    "push r12",
    "push r13",
    "push r14",
    "push r15",

    "mov [rdi], rsp",
    "mov rsp, rsi",

    "pop r15",
    "pop r14",
    "pop r13",
    "pop r12",
    "pop rbp",
    "pop rbx",

    "ret",

    ".size srikesh_context_switch, .-srikesh_context_switch",

    ".global srikesh_task_trampoline",
    ".type srikesh_task_trampoline,@function",

    "srikesh_task_trampoline:",
    "sub rsp, 8",
    "call r12",
    "cli",

    "2:",
    "hlt",
    "jmp 2b",

    ".size srikesh_task_trampoline, .-srikesh_task_trampoline",
);

unsafe extern "C" {
    fn srikesh_context_switch(
        old_rsp: *mut usize,
        new_rsp: usize,
    );

    fn srikesh_task_trampoline();
}

pub fn initialize(
    stack: &mut TaskStack,
    entry: extern "C" fn() -> !,
) -> CpuContext {
    let top = stack.data.as_mut_ptr() as usize + TASK_STACK_SIZE;
    let aligned_top = top & !0xF;

    // Reserve eight words to maintain the required stack alignment.
    let frame = (aligned_top - 8 * core::mem::size_of::<usize>())
        as *mut usize;

    // Initial register frame:
    // r15, r14, r13, r12, rbp, rbx, return address, padding.
    unsafe {
        frame.add(0).write(0);
        frame.add(1).write(0);
        frame.add(2).write(0);
        frame.add(3).write(entry as *const () as usize);
        frame.add(4).write(0);
        frame.add(5).write(0);
        frame.add(6).write(srikesh_task_trampoline as *const () as usize);
        frame.add(7).write(0);
    }

    CpuContext {
        rsp: frame as usize,
    }
}

/// Switch between two initialized CPU contexts.
pub unsafe fn switch(
    old: &mut CpuContext,
    new: &CpuContext,
) {
    unsafe {
        srikesh_context_switch(&mut old.rsp, new.rsp);
    }
}


/// Build the initial register frame consumed by the timer interrupt
/// restore path. The frame layout matches the assembly pop sequence
/// followed by IRETQ.
pub fn initialize_interrupt_frame(
    stack: &mut TaskStack,
    entry: extern "C" fn() -> !,
) -> usize {
    use x86_64::instructions::segmentation::{Segment, CS, SS};

    let top = stack.data.as_mut_ptr() as usize + TASK_STACK_SIZE;
    let initial_rsp = (top & !0xF) - 8;
    let frame = (initial_rsp - 20 * core::mem::size_of::<usize>())
        as *mut usize;

    let cs = CS::get_reg().0 as usize;
    let ss = SS::get_reg().0 as usize;

    unsafe {
        for index in 0..15 {
            frame.add(index).write(0);
        }

        frame.add(15).write(entry as *const () as usize);
        frame.add(16).write(cs);
        frame.add(17).write(0x202);
        frame.add(18).write(initial_rsp);
        frame.add(19).write(ss);
    }

    frame as usize
}


// Start a task from a prepared interrupt frame.
// The frame contains 15 general-purpose registers followed by
// RIP, CS and RFLAGS, matching the timer interrupt restore layout.
global_asm!(
    ".global srikesh_start_interrupt_context",
    ".type srikesh_start_interrupt_context,@function",
    "srikesh_start_interrupt_context:",
    "mov rsp, rdi",
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
    ".size srikesh_start_interrupt_context, .-srikesh_start_interrupt_context",
);

unsafe extern "C" {
    fn srikesh_start_interrupt_context(rsp: usize) -> !;
}

/// Begin execution using a task's initialized interrupt frame.
pub unsafe fn start_interrupt_context(rsp: usize) -> ! {
    unsafe {
        srikesh_start_interrupt_context(rsp);
    }
}
