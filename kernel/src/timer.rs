use core::arch::asm;

const PIT_COMMAND_PORT: u16 = 0x43;
const PIT_CHANNEL0_PORT: u16 = 0x40;

const PIT_BASE_FREQUENCY: u32 = 1_193_182;
const TIMER_FREQUENCY: u32 = 100;

pub fn init() {
    let divisor = (PIT_BASE_FREQUENCY / TIMER_FREQUENCY) as u16;

    unsafe {
        // Configure channel 0: square-wave mode, low byte then high byte.
        asm!(
            "out dx, al",
            in("dx") PIT_COMMAND_PORT,
            in("al") 0x36u8,
            options(nomem, nostack, preserves_flags)
        );

        // Send the divisor's low byte.
        asm!(
            "out dx, al",
            in("dx") PIT_CHANNEL0_PORT,
            in("al") (divisor & 0xFF) as u8,
            options(nomem, nostack, preserves_flags)
        );

        // Send the divisor's high byte.
        asm!(
            "out dx, al",
            in("dx") PIT_CHANNEL0_PORT,
            in("al") (divisor >> 8) as u8,
            options(nomem, nostack, preserves_flags)
        );
    }
}
