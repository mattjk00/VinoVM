pub mod masks {
    pub const IMASK:u64 = 0xFF00_0000_0000_0000;
    pub const PMASK:u64 = 0x00FF_FFFF_FFFF_FFFF;
}

pub mod basic {
    pub const LOADC:u64 = 0x1000_0000_0000_0000;
    pub const STORE:u64 = 0x1100_0000_0000_0000;
    pub const LOAD:u64 =  0x1200_0000_0000_0000;
    pub const QUIT:u64 =  0xFF00_0000_0000_0000;
}

pub mod stack {
    pub const PUSH:u64 = 0x2000_0000_0000_0000;
    pub const POP:u64 =  0x2100_0000_0000_0000;
}

pub mod jumps {
    pub const JA:u64 =  0x3000_0000_0000_0000;
    pub const JZ:u64 =  0x3100_0000_0000_0000;
    pub const JNZ:u64 = 0x3200_0000_0000_0000;
}

pub mod debug {
    pub const DBG_LOG:u64 = 0xF000_0000_0000_0000;
    pub const DBG_STR:u64 = 0xF100_0000_0000_0000;
}