use pic8259::ChainedPics;
use spin::Mutex;

pub const PIC_1_OFFSET: u8 = 0x20;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<Option<ChainedPics>> = Mutex::new(None);

pub fn initialize_pics() {
    unsafe {
        let mut pics = ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET);
        pics.initialize();
          pics.write_masks(0xFE, 0xFF);

        *PICS.lock() = Some(pics);
    }
}

pub fn end_of_interrupt(interrupt_id: u8) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        if let Some(ref mut pics) = *PICS.lock() {
            unsafe {
                pics.notify_end_of_interrupt(interrupt_id);
            }
        }
    });
}
