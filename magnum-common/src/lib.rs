// Opcode constants
pub const OPCODE_NOP: u8 = 0x00;
pub const LOADI_B: u8 = 0x01;
pub const LOADI_2B: u8 = 0x02;
pub const LOADI_4B: u8 = 0x03;
pub const LOADI_8B: u8 = 0x04;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
