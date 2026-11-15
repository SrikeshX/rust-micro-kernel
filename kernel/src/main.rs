#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]
mod cpu;
mod task;
mod ready_queue;
mod scheduler;
mod sync;
mod memory;
mod logger;
mod serial;
mod interrupts;
mod pic;
mod timer;
mod context;
mod interrupt_context;
mod task_control;
mod scheduler_tests;

// Permanently allocated task control blocks.
// Their addresses remain stable for the lifetime of the kernel.
static mut TASK1_TCB: task_control::TaskControlBlock =
    task_control::TaskControlBlock::uninitialized(
        1, 5, preempt_task1_entry
    );

static mut TASK2_TCB: task_control::TaskControlBlock =
    task_control::TaskControlBlock::uninitialized(
        2, 5, preempt_task2_entry
    );


use bootloader_api::{entry_point, BootInfo};
use bootloader_api::info::PixelFormat;
use font8x8::{UnicodeFonts, BASIC_FONTS};

entry_point!(kernel_main);

fn draw_text(
    buffer: &mut [u8],
    width: usize,
    height: usize,
    stride: usize,
    bpp: usize,
    format: PixelFormat,
    text: &str,
    y: usize,
    scale: usize,
) {
    let text_width = text.chars().count() * 8 * scale;
    let start_x = width.saturating_sub(text_width) / 2;

    for (i, ch) in text.chars().enumerate() {
        if let Some(glyph) = BASIC_FONTS.get(ch) {
            for (row, bits) in glyph.iter().enumerate() {
                for col in 0..8 {
                    if (*bits & (1 << col)) != 0 {
                        for sy in 0..scale {
                            for sx in 0..scale {
                                let px = start_x + i * 8 * scale + col * scale + sx;
                                let py = y + row * scale + sy;

                                if px < width && py < height {
                                    let offset =
                                        (py * stride + px) * bpp;

                                    if offset + bpp <= buffer.len() {
                                        match format {
                                            PixelFormat::Bgr | PixelFormat::Rgb => {
                                                if bpp >= 3 {
                                                    buffer[offset] = 255;
                                                    buffer[offset + 1] = 255;
                                                    buffer[offset + 2] = 255;
                                                }
                                            }
                                            PixelFormat::U8 => {
                                                buffer[offset] = 255;
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}




// The boot context stores the kernel's execution state.
static mut BOOT_CONTEXT: context::CpuContext =
    context::CpuContext { rsp: 0 };

// Shared ready queue containing pointers to permanently allocated TCBs.
static mut READY_QUEUE: ready_queue::ReadyQueue =
    ready_queue::ReadyQueue::new();

static CURRENT_TASK: core::sync::atomic::AtomicPtr<task_control::TaskControlBlock> =
    core::sync::atomic::AtomicPtr::new(core::ptr::null_mut());

static PREEMPTION_SLICE: core::sync::atomic::AtomicU64 =
    core::sync::atomic::AtomicU64::new(0);

// Scheduler specialization: configurable quantum and runtime statistics.
const SCHEDULER_QUANTUM_TICKS: u64 = 5;

static CONTEXT_SWITCH_COUNT: core::sync::atomic::AtomicU64 =
    core::sync::atomic::AtomicU64::new(0);

static TASK1_DISPATCH_COUNT: core::sync::atomic::AtomicU64 =
    core::sync::atomic::AtomicU64::new(0);

static TASK2_DISPATCH_COUNT: core::sync::atomic::AtomicU64 =
    core::sync::atomic::AtomicU64::new(0);


pub(crate) fn timer_schedule(frame: usize) -> usize {
    use core::sync::atomic::Ordering;

    let current = CURRENT_TASK.load(Ordering::Relaxed);

    if current.is_null() {
        return frame;
    }

    unsafe {
        (*current).interrupt_rsp = frame;
    }

    let elapsed = PREEMPTION_SLICE.fetch_add(1, Ordering::Relaxed) + 1;

    if elapsed < SCHEDULER_QUANTUM_TICKS {
        return frame;
    }

    PREEMPTION_SLICE.store(0, Ordering::Relaxed);

    unsafe {
        let queue = &mut *core::ptr::addr_of_mut!(READY_QUEUE);

        if (*current).status != task_control::TaskStatus::Running {
            return frame;
        }

        (*current).set_status(task_control::TaskStatus::Ready);

        if queue.enqueue(current).is_err() {
            (*current).set_status(task_control::TaskStatus::Running);
            return frame;
        }

        if let Some(next) = scheduler::dispatch_next(queue) {
            if next != current {
                CONTEXT_SWITCH_COUNT.fetch_add(1, Ordering::Relaxed);

                match (*next).id {
                    1 => { TASK1_DISPATCH_COUNT.fetch_add(1, Ordering::Relaxed); }
                    2 => { TASK2_DISPATCH_COUNT.fetch_add(1, Ordering::Relaxed); }
                    _ => {}
                }
            }

            CURRENT_TASK.store(next, Ordering::Relaxed);
            return (*next).interrupt_rsp;
        }

        (*current).set_status(task_control::TaskStatus::Running);
        CURRENT_TASK.store(current, Ordering::Relaxed);
        frame
    }
}

fn halt_forever(message: &str) -> ! {
    serial::write_str(message);

    loop {
        x86_64::instructions::hlt();
    }
}


extern "C" fn preempt_task1_entry() -> ! {
    use core::sync::atomic::Ordering;

    serial::write_str("\n[PREEMPT] TASK 1 STARTED\n");

    let mut progress: u64 = 0;
    let mut performance_reported = false;

    loop {
        progress = progress.wrapping_add(1);

        if progress >= 10_000_000 && !performance_reported {
            performance_reported = true;

            serial::write_str("\n[PERFORMANCE] Scheduler runtime snapshot\n");

            serial::write_str("[PERFORMANCE] Context switches: ");
            serial::write_u64(CONTEXT_SWITCH_COUNT.load(Ordering::Relaxed));
            serial::write_str("\n");

            serial::write_str("[PERFORMANCE] Task 1 dispatches: ");
            serial::write_u64(TASK1_DISPATCH_COUNT.load(Ordering::Relaxed));
            serial::write_str("\n");

            serial::write_str("[PERFORMANCE] Task 2 dispatches: ");
            serial::write_u64(TASK2_DISPATCH_COUNT.load(Ordering::Relaxed));
            serial::write_str("\n");

            serial::write_str("[PERFORMANCE] Scheduler quantum: ");
            serial::write_u64(SCHEDULER_QUANTUM_TICKS);
            serial::write_str(" ticks\n");
        }

        if progress % 10_000_000 == 0 {
            serial::write_str("[PREEMPT] TASK 1 RUNNING\n");
        }

        core::hint::spin_loop();
    }
}

extern "C" fn preempt_task2_entry() -> ! {
    serial::write_str("\n[PREEMPT] TASK 2 STARTED\n");

    let mut progress: u64 = 0;

    loop {
        progress = progress.wrapping_add(1);

        if progress % 10_000_000 == 0 {
            serial::write_str("[PREEMPT] TASK 2 RUNNING\n");
        }

        core::hint::spin_loop();
    }
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    serial::init();
    x86_64::instructions::interrupts::disable();
    interrupts::init_idt();
    pic::initialize_pics();
    timer::init();
    logger::info("SRIKESH OS kernel started");
    memory::report(boot_info);
    cpu::report();
    let mut first_task = task::Task::new(1, 5);

    logger::info("Task created successfully");
    task::report(&first_task);

    first_task.set_state(task::TaskState::RUNNING);
    task::report(&first_task);

    first_task.set_state(task::TaskState::BLOCKED);
    task::report(&first_task);

    first_task.set_state(task::TaskState::TERMINATED);
    task::report(&first_task);

    logger::info("Legacy queue demonstration replaced by TCB scheduler");
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        let buffer = framebuffer.buffer_mut();

        let bpp = info.bytes_per_pixel;

        // Paint the entire screen black.
        for y in 0..info.height {
            for x in 0..info.width {
                let offset = (y * info.stride + x) * bpp;

                if offset + bpp <= buffer.len() {
                    match info.pixel_format {
                        PixelFormat::Bgr | PixelFormat::Rgb => {
                            buffer[offset] = 0;
                            if bpp > 1 {
                                buffer[offset + 1] = 0;
                            }
                            if bpp > 2 {
                                buffer[offset + 2] = 0;
                            }
                        }
                        PixelFormat::U8 => {
                            buffer[offset] = 0;
                        }
                        _ => {}
                    }
                }
            }
        }

        // Premium minimalist welcome screen.
        let title_y = info.height.saturating_sub(82) / 2;

        draw_text(
            buffer, info.width, info.height, info.stride,
            bpp, info.pixel_format,
            "S R I K E S H", title_y, 3
        );

        draw_text(
            buffer, info.width, info.height, info.stride,
            bpp, info.pixel_format,
            "OS", title_y + 34, 2
        );

        draw_text(
            buffer, info.width, info.height, info.stride,
            bpp, info.pixel_format,
            "WELCOME TO MY SYSTEM", title_y + 62, 2
        );
    }

    logger::info("Starting PIT timer test...");
    x86_64::instructions::interrupts::enable();

    while interrupts::TIMER_TICKS.load(
        core::sync::atomic::Ordering::Relaxed
    ) < 100 {
        x86_64::instructions::hlt();
    }

    x86_64::instructions::interrupts::disable();
    logger::info("PIT timer test passed: received 100 ticks");
    crate::sync::run_demo();
    scheduler_tests::run();
    memory::allocator_demo();

    

logger::info("Initializing interrupt-driven task contexts");

// Initialize Task 1's interrupt stack.
let tcb1_ptr = core::ptr::addr_of_mut!(TASK1_TCB);
let tcb1_pin = unsafe {
    core::pin::Pin::new_unchecked(&mut *tcb1_ptr)
};

if tcb1_pin.initialize_interrupt_context().is_err() {
    halt_forever("[ERROR] Task 1 interrupt context initialization failed\n");
}

// Initialize Task 2's interrupt stack.
let tcb2_ptr = core::ptr::addr_of_mut!(TASK2_TCB);
let tcb2_pin = unsafe {
    core::pin::Pin::new_unchecked(&mut *tcb2_ptr)
};

if tcb2_pin.initialize_interrupt_context().is_err() {
    halt_forever("[ERROR] Task 2 interrupt context initialization failed\n");
}

logger::info("Both interrupt task contexts initialized");

unsafe {
    let frame = (*tcb1_ptr).interrupt_rsp as *const usize;

    serial::write_str("[DEBUG] TASK 1 FRAME WORDS:\n");

    for index in 15..20 {
        serial::write_str("[DEBUG] FRAME[");
        serial::write_hex(index as u64);
        serial::write_str("] = ");
        serial::write_hex(*frame.add(index) as u64);
        serial::write_str("\n");
    }

    use x86_64::instructions::segmentation::{Segment, CS};
    serial::write_str("[DEBUG] CURRENT CS = ");
    serial::write_hex(CS::get_reg().0 as u64);
    serial::write_str("\n");
}

unsafe {
    let task1_rsp = (*tcb1_ptr).interrupt_rsp;
    let task2_rsp = (*tcb2_ptr).interrupt_rsp;

    serial::write_str("[DEBUG] TASK 1 initial frame: ");
    serial::write_hex(task1_rsp as u64);
    serial::write_str("\n");

    serial::write_str("[DEBUG] TASK 1 expected post-IRET RSP: ");
    serial::write_hex(
        (task1_rsp + 20 * core::mem::size_of::<usize>()) as u64
    );
    serial::write_str("\n");

    serial::write_str("[DEBUG] TASK 2 initial frame: ");
    serial::write_hex(task2_rsp as u64);
    serial::write_str("\n");

    serial::write_str("[DEBUG] TASK 2 expected post-IRET RSP: ");
    serial::write_hex(
        (task2_rsp + 20 * core::mem::size_of::<usize>()) as u64
    );
    serial::write_str("\n");
}

// Prepare the ready queue and dispatch the first task.
let first_task = unsafe {
    let queue = &mut *core::ptr::addr_of_mut!(READY_QUEUE);

    let task1 = core::ptr::addr_of_mut!(TASK1_TCB);
    let task2 = core::ptr::addr_of_mut!(TASK2_TCB);

    if queue.enqueue(task1).is_err() {
        core::ptr::null_mut()
    } else if queue.enqueue(task2).is_err() {
        core::ptr::null_mut()
    } else {
        scheduler::dispatch_next(queue)
            .unwrap_or(core::ptr::null_mut())
    }
};

if first_task.is_null() {
    halt_forever("[ERROR] Initial preemptive dispatch failed\n");
}

PREEMPTION_SLICE.store(
    0,
    core::sync::atomic::Ordering::Relaxed
);

CURRENT_TASK.store(
    first_task,
    core::sync::atomic::Ordering::Relaxed
);

// Count the initial dispatch of the first task.
unsafe {
    match (*first_task).id {
        1 => { TASK1_DISPATCH_COUNT.fetch_add(1, core::sync::atomic::Ordering::Relaxed); }
        2 => { TASK2_DISPATCH_COUNT.fetch_add(1, core::sync::atomic::Ordering::Relaxed); }
        _ => {}
    }
}

logger::info("Initial task dispatched");
logger::info("Starting timer-driven Round Robin scheduler");

// The initial synthetic RFLAGS enables interrupts through IRETQ.
unsafe {
    context::start_interrupt_context((*first_task).interrupt_rsp);
}

}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
