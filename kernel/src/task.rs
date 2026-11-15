use crate::serial;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    READY,
    RUNNING,
    BLOCKED,
    TERMINATED,
}

pub struct Task {
    pub id: u64,
    pub state: TaskState,
    pub time_slice_ticks: u64,
}

impl Task {
    pub fn new(id: u64, time_slice_ticks: u64) -> Self {
        Self {
            id,
            state: TaskState::READY,
            time_slice_ticks,
        }
    }
}

pub fn report(task: &Task) {
    serial::write_str("[TASK] Task ID: ");
    write_number(task.id);

    serial::write_str("\n[TASK] State: ");

    let state = match task.state {
        TaskState::READY => "READY",
        TaskState::RUNNING => "RUNNING",
        TaskState::BLOCKED => "BLOCKED",
        TaskState::TERMINATED => "TERMINATED",
    };

    serial::write_str(state);
    serial::write_str("\n[TASK] Time slice: ");
    write_number(task.time_slice_ticks);
    serial::write_str(" ticks\n");
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

impl Task {
    pub fn set_state(&mut self, new_state: TaskState) {
        self.state = new_state;
    }
}
