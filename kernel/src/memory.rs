use bootloader_api::{
    info::{MemoryRegionKind},
    BootInfo,
};

use crate::serial;

fn write_number(mut value: u64) {
    if value == 0 {
        serial::write_str("0");
        return;
    }

    let mut digits = [0u8; 20];
    let mut index = 0;

    while value > 0 {
        digits[index] = b'0' + (value % 10) as u8;
        value /= 10;
        index += 1;
    }

    while index > 0 {
        index -= 1;
        serial::write_byte(digits[index]);
    }
}

pub fn report(boot_info: &BootInfo) {
    let mut usable_bytes = 0u64;
    let mut usable_regions = 0u64;

    for region in boot_info.memory_regions.iter() {
        if region.kind == MemoryRegionKind::Usable {
            usable_bytes += region.end - region.start;
            usable_regions += 1;
        }
    }

    serial::write_str("[MEMORY] Usable RAM: ");
    write_number(usable_bytes / (1024 * 1024));
    serial::write_str(" MB\n");

    serial::write_str("[MEMORY] Usable regions: ");
    write_number(usable_regions);
    serial::write_str("\n");

    serial::write_str("[MEMORY] Kernel size: ");
    write_number(boot_info.kernel_len / 1024);
    serial::write_str(" KB\n");
}


// Simple fixed-size kernel bump allocator.
use core::sync::atomic::{AtomicUsize, Ordering};

const HEAP_SIZE: usize = 64 * 1024;

#[repr(align(4096))]
struct Heap([u8; HEAP_SIZE]);

static mut HEAP: Heap = Heap([0; HEAP_SIZE]);
static HEAP_USED: AtomicUsize = AtomicUsize::new(0);

pub fn allocate(size: usize, align: usize) -> Option<core::ptr::NonNull<u8>> {
    if size == 0 || align == 0 || !align.is_power_of_two() {
        return None;
    }

    let base = core::ptr::addr_of_mut!(HEAP).cast::<u8>() as usize;

    loop {
        let current = HEAP_USED.load(Ordering::Relaxed);
        let address = base.checked_add(current)?;
        let aligned = address.checked_add(align - 1)? & !(align - 1);
        let end = aligned.checked_add(size)?;
        let next = end.checked_sub(base)?;

        if next > HEAP_SIZE {
            return None;
        }

        if HEAP_USED.compare_exchange_weak(
            current, next, Ordering::SeqCst, Ordering::Relaxed
        ).is_ok() {
            return core::ptr::NonNull::new(aligned as *mut u8);
        }
    }
}

pub fn heap_used() -> usize {
    HEAP_USED.load(Ordering::SeqCst)
}

pub fn heap_capacity() -> usize {
    HEAP_SIZE
}

pub fn heap_remaining() -> usize {
    HEAP_SIZE.saturating_sub(heap_used())
}

pub fn allocator_demo() {
    crate::logger::info("Starting memory allocator tests");

    // Test 1: Reject invalid allocation requests.
    if allocate(0, 16).is_some()
        || allocate(32, 0).is_some()
        || allocate(32, 3).is_some()
    {
        crate::logger::info("Allocator test failed: invalid request accepted");
        return;
    }
    crate::logger::info("Allocator test passed: invalid requests rejected");

    // Test 2: Allocate memory with different alignments.
    let first = match allocate(128, 16) {
        Some(ptr) => ptr,
        None => {
            crate::logger::info("Allocator test failed: first allocation");
            return;
        }
    };

    let second = match allocate(256, 64) {
        Some(ptr) => ptr,
        None => {
            crate::logger::info("Allocator test failed: second allocation");
            return;
        }
    };

    if first.as_ptr() as usize % 16 != 0
        || second.as_ptr() as usize % 64 != 0
    {
        crate::logger::info("Allocator test failed: alignment");
        return;
    }
    crate::logger::info("Allocator test passed: memory alignment");

    // Test 3: Ensure the allocations do not overlap.
    let first_start = first.as_ptr() as usize;
    let second_start = second.as_ptr() as usize;

    let first_end = first_start + 128;
    let second_end = second_start + 256;

    if first_start < second_end && second_start < first_end {
        crate::logger::info("Allocator test failed: overlapping allocations");
        return;
    }
    crate::logger::info("Allocator test passed: separate allocations");

    // Test 4: Check capacity and remaining heap space.
    if heap_used() > heap_capacity()
        || heap_remaining() != heap_capacity() - heap_used()
    {
        crate::logger::info("Allocator test failed: heap accounting");
        return;
    }
    crate::logger::info("Allocator test passed: heap accounting");

    // Test 5: Consume the remaining heap and verify exhaustion.
    let remaining = heap_remaining();

    if remaining == 0 || allocate(remaining, 1).is_none() {
        crate::logger::info("Allocator test failed: heap exhaustion");
        return;
    }

    if heap_remaining() != 0 || allocate(1, 1).is_some() {
        crate::logger::info("Allocator test failed: exhaustion boundary");
        return;
    }

    crate::logger::info("Allocator test passed: heap exhaustion");
    crate::logger::info("All memory allocator tests passed");
}
