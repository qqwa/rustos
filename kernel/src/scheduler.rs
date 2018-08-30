#[derive(Debug, Clone)]
pub struct Scheduler {
    current: u64,
    next: u64,
    stack_pointer_base: usize,
    stack_pointer_size: usize,
	processes: [Option<crate::process::Process>; 3],
}

#[allow(dead_code)]
impl Scheduler {
    pub fn new(base: usize, size: usize) -> Scheduler {
        Scheduler {
            current: 0,
            next: 0,
            stack_pointer_base: base,
            stack_pointer_size: size,
			processes: [None, None, None],
        }
    }


    pub fn add(&mut self, _func: fn()) {

    }

    pub fn start(&mut self) -> ! {
        loop {}
    }
}

