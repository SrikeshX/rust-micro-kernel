pub fn info(message: &str) {
    serial::write_str("[INFO] ");
    serial::write_str(message);
    serial::write_str("\n");
}

pub fn warn(message: &str) {
    serial::write_str("[WARN] ");
    serial::write_str(message);
    serial::write_str("\n");
}

pub fn error(message: &str) {
    serial::write_str("[ERROR] ");
    serial::write_str(message);
    serial::write_str("\n");
}

mod serial {
    pub use crate::serial::write_str;
}
