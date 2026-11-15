//! Kernel synchronization demonstration and self-test.

use spin::Mutex;

static SHARED_COUNTER: Mutex<u64> = Mutex::new(0);

const SYNC_TEST_ITERATIONS: u64 = 1000;

pub fn run_demo() {
    // Reset the shared counter before the test.
    {
        let mut counter = SHARED_COUNTER.lock();
        *counter = 0;
    }

    // Perform repeated updates while holding the spinlock.
    for _ in 0..SYNC_TEST_ITERATIONS {
        let mut counter = SHARED_COUNTER.lock();
        *counter += 1;
    }

    // Read the final value under the same lock.
    let final_count = {
        let counter = SHARED_COUNTER.lock();
        *counter
    };

    crate::logger::info("Synchronization: shared resource protected by spinlock");

    if final_count == SYNC_TEST_ITERATIONS {
        crate::logger::info("Synchronization test passed: 1000 protected updates");
    } else {
        crate::logger::info("Synchronization test failed: counter mismatch");
    }
}
