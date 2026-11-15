use crate::task_control::{TaskControlBlock, TaskStatus};

pub const QUEUE_CAPACITY: usize = 8;

pub struct ReadyQueue {
    tasks: [*mut TaskControlBlock; QUEUE_CAPACITY],
    head: usize,
    tail: usize,
    len: usize,
}

impl ReadyQueue {
    pub const fn new() -> Self {
        Self {
            tasks: [core::ptr::null_mut(); QUEUE_CAPACITY],
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    pub fn enqueue(
        &mut self,
        task: *mut TaskControlBlock,
    ) -> Result<(), *mut TaskControlBlock> {
        if task.is_null() || self.len == QUEUE_CAPACITY {
            return Err(task);
        }

        let is_ready = unsafe {
            (*task).status == TaskStatus::Ready
        };

        if !is_ready {
            return Err(task);
        }

        self.tasks[self.tail] = task;
        self.tail = (self.tail + 1) % QUEUE_CAPACITY;
        self.len += 1;

        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<*mut TaskControlBlock> {
        if self.len == 0 {
            return None;
        }

        let task = self.tasks[self.head];
        self.tasks[self.head] = core::ptr::null_mut();
        self.head = (self.head + 1) % QUEUE_CAPACITY;
        self.len -= 1;

        if task.is_null() {
            None
        } else {
            Some(task)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_full(&self) -> bool {
        self.len == QUEUE_CAPACITY
    }
}
