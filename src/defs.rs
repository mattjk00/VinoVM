pub mod masks {
    pub const IMASK:i64 =   0xFF00_0000;
    pub const PMASK:i64 =   0x00FF_FFFF;
}

pub mod basic {
    pub const LOADC:i64 =   0x1000_0000;
    pub const STORE:i64 =   0x1100_0000;
    pub const LOAD:i64 =    0x1200_0000;
    pub const QUIT:i64 =    0xFF00_0000;
}

pub mod stack {
    pub const PUSH:i64 =    0x2000_0000;
    pub const POP:i64 =     0x2100_0000;
}

pub mod jumps {
    pub const JA:i64 =      0x3000_0000;
    pub const JZ:i64 =      0x3100_0000;
    pub const JN:i64 =     0x3200_0000;
    pub const JP:i64 =     0x3300_0000;
}

pub mod funcs {
    pub const CALL:i64 =    0x4000_0000;
    pub const RETURN:i64 =  0x4100_0000;
}

pub mod math {
    pub const ADD:i64 =     0x5000_0000;
    pub const MULT:i64 =    0x5100_0000;
    pub const SUB:i64 =     0x5200_0000;
    pub const DIV:i64 =     0x5300_0000;
}

pub mod debug {
    pub const DBG_LOG:i64 = 0xF000_0000;
    pub const DBG_STR:i64 = 0xF100_0000;
}