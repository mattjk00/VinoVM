//mod defs;
use crate::defs::basic::*;
use crate::defs::masks::*;
use crate::defs::debug::*;
use crate::defs::stack::*;
use crate::defs::math::*;
use crate::defs::jumps::*;
use crate::defs::funcs::*;
//mod debugger;
use crate::debugger::Debugger;

pub struct Machine {
    pub ir:     u64,
    pub ac:     i64,
    pub mdr:    i64,
    pub mar:    u64,
    pub sp:     u64,

    pub rr:             Vec<u64>,
    pub memory:         Vec<i64>,
    pub instructions:   Vec<i64>,
    pub stack:          Vec<i64>,
    pub stack_table:    Vec<i64>,

    pub running:bool,
    pub trace:  bool
}

impl Machine {
    pub fn new_instance(mem_size:usize, ins_size:usize, stack_size:usize) -> Machine {
        Machine { 
            ir:0, mdr:0, mar:0, sp:0, ac:0, rr:Vec::with_capacity(stack_size),
            memory:vec![0; mem_size], instructions:Vec::with_capacity(ins_size), stack:Vec::with_capacity(stack_size),
            stack_table:Vec::with_capacity(stack_size),
            running:false, trace:false
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

        self.vmprint(format!("{:x}", instr));

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
        else if opcode == STOP {
            self.stop();
        }

        // STACK
        else if opcode == PUSH {
            self.push();
        }
        else if opcode == POP {
            self.pop();
        }
        else if opcode == PUSH_TABLE {
            self.push_table();
        }
        else if opcode == LOAD_TABLE {
            self.load_table(param as usize);
        }
        else if opcode == STORE_TABLE {
            self.store_table(param as usize);
        }

        // MATH
        else if opcode == ADD {
            self.add();
        }
        else if opcode == SUB {
            self.sub();
        }
        else if opcode == MULT {
            self.mult();
        }
        else if opcode == DIV {
            self.div();
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
        else if opcode == JP {
            self.jump_positive(param as u64);
        }

        // Functions
        else if opcode == CALL {
            self.call(param as u64);
        }
        else if opcode == RETURN {
            self.ret();
        }

        // DEBUG
        else if opcode == DBG_LOG {
            Debugger::print(self.ac);
        }
        else if opcode == DBG_STR {
            Debugger::print_str(self.memory.clone(), self.ac as usize);
        }
        else {
            println!("Invalid Bytecode: {:#x}", opcode);
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
            None => println!("Stack Error!")
        };
        self.sp -= 1;
    }

    fn push_table(&mut self) {
        self.stack.push(self.ac);
        self.stack_table.push((self.stack.len()-1) as i64);
    }

    fn load_table(&mut self, addr:usize) {
        let val = self.stack[self.stack_table[addr] as usize];
        self.ac = val;
    }

    fn store_table(&mut self, addr:usize) {
        self.stack[self.stack_table[addr] as usize] = self.ac;
    }

    fn add(&mut self) {
        let param = self.ac;
        self.pop();
        self.ac = self.ac + param;
    }

    fn sub(&mut self) {
        let param = self.ac;
        self.pop();
        self.ac = self.ac - param;
    }

    fn mult(&mut self) {
        let param = self.ac;
        self.pop();
        self.ac = self.ac * param;
    }

    fn div(&mut self) {
        let param = self.ac;
        self.pop();
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

    fn jump_positive(&mut self, param:u64) {
        if self.ac > 0 {
            self.ir = param - 1;
        }
    }

    fn call(&mut self, addr:u64) {
        self.rr.push(self.ir as u64);
        self.ir = addr - 1;
    }

    fn ret(&mut self) {
        //self.ir = self.rr;
        let radr = self.rr.pop();
        match radr {
            Some(r) => self.ir = r,
            None => { println!("Return Error! Can't find home :("); self.quit(); }
        };
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

        self.vmprint(format!("Loaded {} words into {} instructions and {} starting memory slots.", ins.len(), self.instructions.len(), memindex));
    }

    fn stop(&mut self) {
        self.running = false;
        
        self.vmprint(String::from("Bytecode Stopped."));
    }

    pub fn quit(&mut self) {
        self.running = false;
        self.vmprint(String::from("VM Exit!"));
    }

    fn vmprint(&self, msg:String) {
        if self.trace {
            println!("{}", msg);
        }
    }
}