// Instruction opcodes
pub const OPCODE_NOP: u8 = 0x00;
pub const OPCODE_LOADI_B: u8 = 0x01;
pub const OPCODE_LOADI_2B: u8 = 0x02;
pub const OPCODE_LOADI_4B: u8 = 0x03;
pub const OPCODE_LOADI_8B: u8 = 0x04;
pub const OPCODE_LOAD: u8 = 0x10;
pub const OPCODE_LOADS: u8 = 0x11;
pub const OPCODE_STORE: u8 = 0x15;
pub const OPCODE_POP: u8 = 0x20;
pub const OPCODE_FUNC_B: u8 = 0x30;
pub const OPCODE_SYS: u8 = 0x40;

// Function opcodes
pub const OPCODE_ADD: u8 = 0x00;
pub const OPCODE_SUB: u8 = 0x01;
pub const OPCODE_MUL: u8 = 0x02;
pub const OPCODE_DIV: u8 = 0x03;

// System call opcodes
pub const OPCODE_PUT_B: u8 = 0x00;
pub const OPCODE_PUT_C: u8 = 0x10;
