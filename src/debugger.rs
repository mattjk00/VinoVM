
use crate::machine::Machine;

pub struct Debugger;

impl Debugger {
    pub fn print(v:u64) {
        println!("vn: {}\t{:#x}\t{:#b}", v, v, v);
    }
    pub fn print_ac(m:Machine) {
        println!("{}", m.ac);
    }
    pub fn print_instructions(m:Machine) {
        for i in 0..m.instructions.len() {
            println!("{}: {:#x}", i, m.instructions[i]);
        }
    }
    pub fn print_memory(m:Machine) {
        for i in 0..m.memory.len() {
            println!("{}: {:#x}", i, m.memory[i]);
        }
    }

    pub fn print_memory_range(m:Machine, a:usize, b:usize) {
        for i in a..b {
            println!("{}: {:#x}", i, m.memory[i]);
        }
    }
}