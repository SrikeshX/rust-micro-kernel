use crate::logger;
use crate::ready_queue::{ReadyQueue, QUEUE_CAPACITY};
use crate::scheduler;
use crate::task_control::{TaskControlBlock, TaskStatus};

extern "C" fn test_task_entry() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

static mut TEST_TASKS: [TaskControlBlock; 9] = [
    const { TaskControlBlock::uninitialized(99, 5, test_task_entry) };
    9
];

pub fn run() {
    logger::info("Starting scheduler self-tests");

    let mut passed = true;
    let queue = &mut ReadyQueue::new();

    if !queue.is_empty() || scheduler::has_ready_tasks(queue) {
        logger::info("Scheduler test failed: empty queue");
        passed = false;
    } else {
        logger::info("Scheduler test passed: empty queue");
    }

    let tasks = core::ptr::addr_of_mut!(TEST_TASKS)
        .cast::<TaskControlBlock>();

    unsafe {
        for index in 0..9 {
            (*tasks.add(index)).set_status(TaskStatus::Ready);
        }

        let task1 = tasks;
        let task2 = tasks.add(1);

        if queue.enqueue(core::ptr::null_mut()).is_ok() {
            logger::info("Scheduler test failed: null task rejection");
            passed = false;
        } else {
            logger::info("Scheduler test passed: null task rejection");
        }

        (*task1).set_status(TaskStatus::Running);

        if queue.enqueue(task1).is_ok() {
            logger::info("Scheduler test failed: invalid state rejection");
            passed = false;
        } else {
            logger::info("Scheduler test passed: invalid state rejection");
        }

        (*task1).set_status(TaskStatus::Ready);

        if queue.enqueue(task1).is_err() || queue.enqueue(task2).is_err() {
            logger::info("Scheduler test failed: FIFO setup");
            passed = false;
        } else {
            let first = scheduler::dispatch_next(queue);

            if first != Some(task1) || (*task1).status != TaskStatus::Running {
                logger::info("Scheduler test failed: FIFO first dispatch");
                passed = false;
            } else {
                logger::info("Scheduler test passed: FIFO first dispatch");
            }

            if scheduler::time_slice_expired(task1, queue).is_err()
                || (*task1).status != TaskStatus::Ready
            {
                logger::info("Scheduler test failed: time-slice expiration");
                passed = false;
            } else {
                logger::info("Scheduler test passed: time-slice expiration");
            }

            let second = scheduler::dispatch_next(queue);

            if second != Some(task2) {
                logger::info("Scheduler test failed: FIFO second dispatch");
                passed = false;
            } else {
                logger::info("Scheduler test passed: FIFO second dispatch");
            }
        }

        let invalid_slice = scheduler::time_slice_expired(
            core::ptr::null_mut(),
            queue,
        );

        if invalid_slice.is_ok() {
            logger::info("Scheduler test failed: null time-slice task");
            passed = false;
        } else {
            logger::info("Scheduler test passed: null time-slice task");
        }

        // Reset every test task before the independent capacity test.
        for index in 0..9 {
            (*tasks.add(index)).set_status(TaskStatus::Ready);
        }

        let capacity_queue = &mut ReadyQueue::new();
        let mut capacity_ok = true;

        for index in 0..QUEUE_CAPACITY {
            if capacity_queue.enqueue(tasks.add(index)).is_err() {
                capacity_ok = false;
                break;
            }
        }

        if !capacity_ok
            || !capacity_queue.is_full()
            || capacity_queue.len() != QUEUE_CAPACITY
            || capacity_queue.enqueue(tasks.add(8)).is_ok()
        {
            logger::info("Scheduler test failed: queue capacity");
            passed = false;
        } else {
            logger::info("Scheduler test passed: queue capacity");
        }
    }

    if passed {
        logger::info("All scheduler self-tests passed");
    } else {
        logger::info("Scheduler self-tests FAILED");
    }
}
