use crate::context::{self, CpuContext, TaskStack};
use core::marker::PhantomPinned;
use core::pin::Pin;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Ready,
    Running,
    Blocked,
    Terminated,
}

pub struct TaskControlBlock {
    pub id: u64,
    pub status: TaskStatus,
    pub time_slice_ticks: u64,
    pub context: CpuContext,

    // Stack pointer reserved for a future timer-interrupt context.
    pub interrupt_rsp: usize,

    // Separate stack reserved for interrupt-driven task execution.
    interrupt_stack: TaskStack,

    // Existing cooperative execution stack.
    stack: TaskStack,
    entry: extern "C" fn() -> !,
    _pin: PhantomPinned,
}

impl TaskControlBlock {
    // Create the TCB without initializing its stack.
    // Its context is initialized only after it reaches stable memory.
    pub const fn uninitialized(
        id: u64,
        time_slice_ticks: u64,
        entry: extern "C" fn() -> !,
    ) -> Self {
        Self {
            id,
            status: TaskStatus::Ready,
            time_slice_ticks,
            context: CpuContext { rsp: 0 },
            interrupt_rsp: 0,
            interrupt_stack: TaskStack::new(),
            stack: TaskStack::new(),
            entry,
            _pin: PhantomPinned,
        }
    }

    // Initialize the task stack at its permanent memory location.
    pub fn initialize_context(
        self: Pin<&mut Self>,
    ) -> Result<(), &'static str> {
        let this = unsafe { self.get_unchecked_mut() };

        if this.context.rsp != 0 {
            return Err("Task context is already initialized");
        }

        this.context = context::initialize(
            &mut this.stack,
            this.entry,
        );

        Ok(())
    }

    pub fn initialize_interrupt_context(
        self: Pin<&mut Self>,
    ) -> Result<(), &'static str> {
        let this = unsafe { self.get_unchecked_mut() };

        if this.interrupt_rsp != 0 {
            return Err("Interrupt context is already initialized");
        }

        this.interrupt_rsp = context::initialize_interrupt_frame(
            &mut this.interrupt_stack,
            this.entry,
        );

        Ok(())
    }

    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }

    pub fn is_ready(&self) -> bool {
        self.status == TaskStatus::Ready
    }

    pub fn is_initialized(&self) -> bool {
        self.context.rsp != 0
    }
}
