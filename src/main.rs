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
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
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
    /*let is = vec![
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
    ];*/
    //let is = [0x10000006', '0x22000000', '0x10000002', '0x20000000', '0x10000002', '0x51000000', '0x20000000', '0x10000005', '0x50000000', '0x20000000', '0x23000000', '0x20000000', '0x21000000', '0x50000000', '0x24000000', '0xf0000000', '0xff000000]
    
    let mut f = File::open("test.vino")?;
    let mut is:Vec<i64> = vec![];
    let mut bytes:Vec<u8> = Vec::new();
    f.read_to_end(&mut bytes);
    for i in (0..bytes.len()).step_by(4) {
        let int64:i64 = ((bytes[i] as i64) << 24) |
                        ((bytes[i+1] as i64) << 16) |
                        ((bytes[i+2] as i64) << 8) |
                        ((bytes[i+3] as i64) << 0);
        is.push(int64);
        //println!("{:x}", int64);
    }
    //is.insert(is.len()-2, DBG_LOG);

    let now = Instant::now();

    machine.load_instructions(is);
    //machine.trace = true;
    machine.start();
    println!("Completed in {} seconds.", now.elapsed().as_secs_f32());
    //Debugger::print_instructions(machine);
    //Debugger::print_stack_vars(machine);
    
    Ok(())

    
    //Debugger::print_memory_range(machine, 0, 5);
}