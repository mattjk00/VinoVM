//extern crate defs;
mod defs;
use defs::*;

struct Machine {
    ir:     u64,
    ac:     u64,
    mdr:    u64,
    mir:    u64,
    sp:     u64,

    memory:         Vec<i64>,
    instructions:   Vec<u64>,
    stack:          Vec<u64>,

    running:bool
}

impl Machine {
    fn new_instance() -> Machine {
        Machine { 
            ir:0, mdr:0, mir:0, sp:0, ac:0,
            memory:vec![], instructions:vec![], stack:vec![],
            running:false
        }
    }

    fn start(&mut self) {
        self.running = true;
        self.read();
    }

    fn incr(&mut self) {
        self.ir += 1;
        if self.ir < self.instructions.len() as u64 {
            self.read();
        } else {
            self.ir = self.instructions.len() as u64;
            self.running = false;
        }
    }

    fn read(&mut self) {
        let instr = self.instructions[self.ir as usize];
        let opcode = instr & IMASK;
        let param = instr & PMASK;
        if opcode == LOADC {
            self.loadc(param);
        }
        if opcode == DBG_LOG {
            Debugger::print(self.ac);
        }

        self.incr();
    }

    fn loadc(&mut self, param:u64) {
        self.ac = param;
    }

    fn load_instructions(&mut self, mut ins:Vec<u64>) {
        self.instructions.append(&mut ins);
    }
}

struct Debugger;
impl Debugger {
    fn print(v:u64) {
        println!("vn: {}\t{:#x}\t{:#b}", v, v, v);
    }
    fn print_ac(m:Machine) {
        println!("{}", m.ac);
    }
    fn print_instructions(m:Machine) {
        for i in 0..m.instructions.len() {
            println!("{}: {:#x}", i, m.instructions[i]);
        }
    }
}

fn main() {
    let mut machine = Machine::new_instance();

    let is = vec![LOADC + 0b11, DBG_LOG];
    machine.load_instructions(is);
    machine.start();

    //Debugger::print_instructions(machine);
}
