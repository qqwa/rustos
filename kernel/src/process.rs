#[derive(Default, Debug, Copy, Clone)]
pub struct Process {
    pub registers: ::arch::exceptions::frame::Frame,
    pub active: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum ProcessState {
    Running,
    Ready,
    Blocked,
    Killing,
}

