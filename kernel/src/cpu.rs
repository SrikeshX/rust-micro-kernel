use crate::serial;
use core::arch::x86_64::__cpuid;
use x86_64::instructions::segmentation::{Segment, CS};

pub fn report() {
    let cs_selector = CS::get_reg().0;

    serial::write_str("[CPU] Active CS selector: ");
    write_number(cs_selector as u64);
    serial::write_str("\n");

    let basic = unsafe { __cpuid(0) };

    let mut vendor = [0u8; 12];

    vendor[0..4].copy_from_slice(&basic.ebx.to_le_bytes());
    vendor[4..8].copy_from_slice(&basic.edx.to_le_bytes());
    vendor[8..12].copy_from_slice(&basic.ecx.to_le_bytes());

    serial::write_str("[CPU] Manufacturer: ");

    for byte in vendor {
        serial::write_byte(byte);
    }

    serial::write_str("\n");

    let features = unsafe { __cpuid(1) };

    serial::write_str("[CPU] Family/Model information detected\n");

    serial::write_str("[CPU] Maximum basic CPUID leaf: ");
    write_number(basic.eax as u64);
    serial::write_str("\n");

    serial::write_str("[CPU] Processor signature: ");
    write_number(features.eax as u64);
    serial::write_str("\n");
}

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
