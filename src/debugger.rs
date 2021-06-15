
use crate::machine::Machine;

pub struct Debugger;

impl Debugger {
    pub fn print(v:i64) {
        println!("vn> {}\t{:#x}", v, v);
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

    pub fn print_str(mem:Vec<i64>, start:usize) {
        
        let mut next = mem[start];
        let mut i = start;

        while next != 0 {
            let c = char::from_u32(next as u32);
            
            match c {
                Some(ch) => print!("{}", ch),
                None => print!("?")
            };
            
            i += 1;
            next = mem[i];
            
        }
        println!();
    }
}