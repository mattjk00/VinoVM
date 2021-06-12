//mod defs;
use crate::defs::*;
//mod debugger;
use crate::debugger::Debugger;

pub struct Machine {
    pub ir:     u64,
    pub ac:     u64,
    pub mdr:    u64,
    pub mar:    u64,
    pub sp:     u64,

    pub memory:         Vec<u64>,
    pub instructions:   Vec<u64>,
    pub stack:          Vec<u64>,

    pub running:bool
}

impl Machine {
    pub fn new_instance(mem_size:usize, ins_size:usize, stack_size:usize) -> Machine {
        Machine { 
            ir:0, mdr:0, mar:0, sp:0, ac:0,
            memory:vec![0; mem_size], instructions:Vec::with_capacity(ins_size), stack:Vec::with_capacity(stack_size),
            running:false
        }
    }

    pub fn start(&mut self) {
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
        else if opcode == STORE {
            self.store(param as usize);
        }
        else if opcode == LOAD {
            self.load(param as usize);
        }
        else if opcode == DBG_LOG {
            Debugger::print(self.ac);
        }
        else if opcode == DBG_STR {
            Debugger::print_str(self.memory.clone(), self.ac as usize);
        }
        else if opcode == QUIT {
            self.quit();
        }
        
        if self.running {
            self.incr();
        }
    }
    
    fn store(&mut self, addr:usize) {
        self.memory[addr] = self.ac;
    }

    fn loadc(&mut self, param:u64) {
        self.ac = param;
    }
    
    fn load(&mut self, addr:usize) {
        self.ac = self.memory[addr];
    }

    pub fn load_instructions(&mut self, mut ins:Vec<u64>) {
        
        let mut loading_i = true;
        let mut memindex = 0;
        let mut skip = false;
        for mut i in 0..ins.len() {
            let next = ins[i];
            skip = false;

            if loading_i && next == QUIT {
                loading_i = false;
                skip = true;
            }

            if loading_i {
                self.instructions.push(next);
            } else if skip == false {
                self.memory[memindex] = next;
                memindex += 1;
            }
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
        println!("VM Exit!");
    }
}