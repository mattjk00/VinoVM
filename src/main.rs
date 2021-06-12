//extern crate defs;
mod machine;
mod defs;
mod debugger;
use machine::Machine;
use defs::*;
use debugger::Debugger;

fn main() {
    let mut machine = Machine::new_instance(256, 256, 256);

    let is = vec![LOAD + 0b01, STORE + 0b100, DBG_LOG, QUIT, 1, 2];
    machine.load_instructions(is);
    machine.start();

    //Debugger::print_instructions(machine);
    Debugger::print_memory_range(machine, 0, 5);
}
