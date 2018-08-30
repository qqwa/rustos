#[derive(Default, Debug, Copy, Clone)]
pub struct Process {
    pub registers: crate::arch::exceptions::frame::Frame,
    pub active: bool,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum ProcessState {
    Running,
    Ready,
    Blocked,
    Killing,
}

