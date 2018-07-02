use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct Scheduler {
    current: u64,
    next: u64,
    stack_pointer_base: usize,
    stack_pointer_size: usize,
	processes: Vec<::process::Process>,
}

impl Scheduler {
    pub fn new(base: usize, size: usize) -> Scheduler {
        Scheduler {
            current: 0,
            next: 0,
            stack_pointer_base: base,
            stack_pointer_size: size,
			processes: Vec::new(),
        }
    }

    pub fn add(&mut self, func: fn()) {
        unsafe {
			let length = self.processes.len();
            self.processes.push(
                ::process::Process {
                    registers: ::arch::exceptions::frame::Frame::new(
						length as u64,
						func as *const fn() as usize as u64,
						(self.stack_pointer_base + self.stack_pointer_size * (length)) as u64),
                    active: false,
                }
            );
        }
    }

    pub fn start(&mut self) {
        println!("Scheduler started");
        unsafe {
            let proc1 = self.processes[0];
            println!("{}", proc1.registers.sp % 16);
            // println!("{:#x?}", PROCESSES);
        }
        loop {}
    }
}

