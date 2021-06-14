//mod defs;
use crate::defs::basic::*;
use crate::defs::masks::*;
use crate::defs::debug::*;
use crate::defs::stack::*;
use crate::defs::math::*;
use crate::defs::jumps::*;
//mod debugger;
use crate::debugger::Debugger;

pub struct Machine {
    pub ir:     u64,
    pub ac:     i64,
    pub mdr:    i64,
    pub mar:    u64,
    pub sp:     u64,

    pub memory:         Vec<i64>,
    pub instructions:   Vec<i64>,
    pub stack:          Vec<i64>,

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

        // BASIC
        if opcode == LOADC {
            self.loadc(param);
        }
        else if opcode == STORE {
            self.store(param as usize);
        }
        else if opcode == LOAD {
            self.load(param as usize);
        }
        else if opcode == QUIT {
            self.quit();
        }

        // STACK
        else if opcode == PUSH {
            self.push();
        }
        else if opcode == POP {
            self.pop();
        }

        // MATH
        else if opcode == ADD {
            self.add(param);
        }
        else if opcode == SUB {
            self.sub(param);
        }
        else if opcode == MULT {
            self.mult(param);
        }
        else if opcode == DIV {
            self.div(param);
        }

        // JUMPS
        else if opcode == JA {
            self.jump_always(param as u64);
        }
        else if opcode == JZ {
            self.jump_zero(param as u64);
        }
        else if opcode == JN {
            self.jump_negative(param as u64);
        }

        // DEBUG
        else if opcode == DBG_LOG {
            Debugger::print(self.ac);
        }
        else if opcode == DBG_STR {
            Debugger::print_str(self.memory.clone(), self.ac as usize);
        }
        
        
        if self.running {
            self.incr();
        }
    }
    
    fn store(&mut self, addr:usize) {
        self.memory[addr] = self.ac;
    }

    fn loadc(&mut self, param:i64) {
        self.ac = param;
    }
    
    fn load(&mut self, addr:usize) {
        self.ac = self.memory[addr];
    }

    fn push(&mut self) {
        self.stack.push(self.ac);
        self.sp += 1;
    }

    fn pop(&mut self) {
        let popvalue = self.stack.pop();
        match popvalue {
            Some(v) => self.ac = v,
            None => println!("ERROR")
        };
        self.sp -= 1;
    }

    fn add(&mut self, param:i64) {
        self.ac = self.ac + param;
    }

    fn sub(&mut self, param:i64) {
        self.ac = self.ac - param;
    }

    fn mult(&mut self, param:i64) {
        self.ac = self.ac * param;
    }

    fn div(&mut self, param:i64) {
        self.ac = self.ac / param;
    }

    fn jump_always(&mut self, param:u64) {
        self.ir = param - 1;
    }

    fn jump_zero(&mut self, param:u64) {
        if self.ac == 0 {
            self.ir = param - 1;
        }
    }

    fn jump_negative(&mut self, param:u64) {
        if self.ac < 0 {
            self.ir = param - 1;
        }
    }

    pub fn load_instructions(&mut self, ins:Vec<i64>) {
        
        let mut loading_i = true;
        let mut memindex = 0;
        let mut skip;
        for i in 0..ins.len() {
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