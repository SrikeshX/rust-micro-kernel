use crate::ready_queue::ReadyQueue;
use crate::task_control::{TaskControlBlock, TaskStatus};

/// Select the next ready task and mark it as running.
pub fn dispatch_next(
    queue: &mut ReadyQueue,
) -> Option<*mut TaskControlBlock> {
    let task = queue.dequeue()?;

    if task.is_null() {
        return None;
    }

    unsafe {
        if (*task).status != TaskStatus::Ready {
            return None;
        }

        (*task).set_status(TaskStatus::Running);
    }

    Some(task)
}

/// Return a task to the ready queue after its time slice.
pub fn time_slice_expired(
    task: *mut TaskControlBlock,
    queue: &mut ReadyQueue,
) -> Result<(), *mut TaskControlBlock> {
    if task.is_null() {
        return Err(task);
    }

    unsafe {
        if (*task).status != TaskStatus::Running {
            return Err(task);
        }

        (*task).set_status(TaskStatus::Ready);
    }

    queue.enqueue(task)
}

/// Check whether another ready task is available.
pub fn has_ready_tasks(queue: &ReadyQueue) -> bool {
    !queue.is_empty()
}
