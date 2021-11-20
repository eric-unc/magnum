// Instruction opcodes
pub const OPCODE_NOP: u8 = 0x00;
pub const LOADI_B: u8 = 0x01;
pub const LOADI_2B: u8 = 0x02;
pub const LOADI_4B: u8 = 0x03;
pub const LOADI_8B: u8 = 0x04;
pub const LOAD: u8 = 0x10;
pub const LOADS: u8 = 0x11;
pub const STORE: u8 = 0x15;
pub const POP: u8 = 0x20;
pub const FUNC_B: u8 = 0x30;
pub const SYS: u8 = 0x40;

// Function opcodes
pub const ADD: u8 = 0x00;
pub const SUB: u8 = 0x01;
pub const MUL: u8 = 0x02;
pub const DIV: u8 = 0x03;

// System call opcodes
pub const PUT_B: u8 = 0x00;
pub const PUT_C: u8 = 0x10;
