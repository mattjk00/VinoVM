#![feature(assoc_char_funcs)]
mod machine;
mod defs;
mod debugger;
use machine::Machine;
use defs::*;
use debugger::Debugger;

fn main() {
    let mut machine = Machine::new_instance(256, 256, 256);

    let is = vec![LOAD + 0b01, STORE + 0b101, DBG_LOG, LOADC + 0b10, DBG_STR, QUIT, 1, 2, 'H' as u64, 'i' as u64, 0];
    machine.load_instructions(is);
    machine.start();

    //Debugger::print_instructions(machine);
    Debugger::print_memory_range(machine, 0, 5);
}
