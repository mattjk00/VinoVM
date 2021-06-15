#![feature(assoc_char_funcs)]
#![allow(dead_code)]
mod machine;
mod defs;
mod debugger;
use machine::Machine;
use crate::defs::basic::*;
use crate::defs::debug::*;
use crate::defs::stack::*;
use crate::defs::math::*;
use crate::defs::jumps::*;
use crate::defs::funcs::*;
use debugger::Debugger;
use std::time::{Duration, Instant};

fn main() {
    let mut machine = Machine::new_instance(256, 256, 256);

    //let is = vec![LOAD + 0b01, STORE + 0b101, LOADC + 0b10, DBG_STR, QUIT, 1, 2, 'H' as i64, 'i' as i64, 0];
    /*let is:Vec<i64> = vec![
        LOAD + 4, 
        SUB + 24,
        JN + 6,
        LOADC,
        DBG_STR, 
        DBG_LOG, 
        QUIT, 
        'P' as i64,
        'o' as i64,
        's' as i64,
        0,
        25, 
        0, 
        0
    ];*/
    let is = vec![
        CALL + 9,
        CALL + 9,
        CALL + 9,
        DBG_LOG,
        STOP,
        //
        LOADC + 5,
        SUB,
        STORE,
        RETURN,

        LOAD,
        PUSH,
        CALL + 5,
        RETURN,

        QUIT,
        0
    ];
    
    let now = Instant::now();

    machine.load_instructions(is);
    
    machine.start();
    println!("Completed in {} seconds.", now.elapsed().as_secs_f32());

    //Debugger::print_instructions(machine);
    //Debugger::print_memory_range(machine, 0, 5);
}