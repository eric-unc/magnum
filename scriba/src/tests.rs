use std::path::Path;
use std::fs;
use std::io::Read;
use crate::{Magna, SystemCall};
use crate::Instruction;
use magnum_common::*;

#[test]
/// For lack of a better test name
fn i_know_what_le_means() {
	let num = 0x100u16;
	let bytes = num.to_le_bytes();
	assert_eq!(bytes, [0, 1]);
}

#[test]
fn one_plus_one() {
	let path = "one-plus-one.magna";

	// Required if test fails, then the file won't be deleted at the end.
	if Path::new(path).exists() {
		fs::remove_file(path).unwrap();
	}

	let mut magna = Magna::new();

	let insts = [
		Instruction::LoadIB(1),
		Instruction::LoadIB(1),
		Instruction::Sys(SystemCall::PutB),
		Instruction::LoadIB('\n' as u8),
		Instruction::Sys(SystemCall::PutC),
		Instruction::Hlt
	];

	for i in insts {
		magna.add_inst(i);
	}

	magna.write_file(path).unwrap();

	let mut file = fs::File::open(path).unwrap();

	let mut file_sig = [0u8; 3];
	file.by_ref().take(3).read(&mut file_sig).unwrap();
	assert_eq!(file_sig, "MVM".as_bytes());

	let mut version = [0u8];
	file.by_ref().take(3).read(&mut version).unwrap();
	assert_eq!(version, [0u8]);

	let mut text_loc = [0u8; 8];
	file.by_ref().take(8).read(&mut text_loc).unwrap();
	assert_eq!(usize::from_le_bytes(text_loc), 0x35);

	let mut text_size = [0u8; 8];
	file.by_ref().take(8).read(&mut text_size).unwrap();
	assert_eq!(usize::from_le_bytes(text_size), insts.len());

	// Note that there is not any global data in this example.
	let mut readonly_loc = [0u8; 8];
	file.by_ref().take(8).read(&mut readonly_loc).unwrap();
	assert_eq!(usize::from_le_bytes(readonly_loc), 0x35 + insts.len());

	let mut readonly_size = [0u8; 8];
	file.by_ref().take(8).read(&mut readonly_size).unwrap();
	assert_eq!(usize::from_le_bytes(readonly_size), 0);

	let mut init_writable_loc = [0u8; 8];
	file.by_ref().take(8).read(&mut init_writable_loc).unwrap();
	assert_eq!(usize::from_le_bytes(init_writable_loc), 0x35 + insts.len());

	let mut init_writable_size = [0u8; 8];
	file.by_ref().take(8).read(&mut init_writable_size).unwrap();
	assert_eq!(usize::from_le_bytes(init_writable_size), 0);

	let mut uninit_writable_size = [0u8; 8];
	file.by_ref().take(8).read(&mut uninit_writable_size).unwrap();
	assert_eq!(usize::from_le_bytes(uninit_writable_size), 0);

	// Our instructions
	let mut loadi_b_1 = [0u8; 4];
	file.by_ref().take(4).read(&mut loadi_b_1).unwrap();
	assert_eq!(u32::from_le_bytes(loadi_b_1), ((OPCODE_LOADI_B as u32) << (8 * 3)) + 1);

	let mut loadi_b_2 = [0u8; 4];
	file.by_ref().take(4).read(&mut loadi_b_2).unwrap();
	assert_eq!(u32::from_le_bytes(loadi_b_2), ((OPCODE_LOADI_B as u32) << (8 * 3)) + 1);

	let mut sys1 = [0u8; 4];
	file.by_ref().take(4).read(&mut sys1).unwrap();
	assert_eq!(u32::from_le_bytes(sys1), ((OPCODE_SYS as u32) << (8 * 3)) + OPCODE_PUT_B as u32);

	let mut loadi_b_3 = [0u8; 4];
	file.by_ref().take(4).read(&mut loadi_b_3).unwrap();
	assert_eq!(u32::from_le_bytes(loadi_b_3), ((OPCODE_LOADI_B as u32) << (8 * 3)) + '\n' as u32);

	let mut sys2 = [0u8; 4];
	file.by_ref().take(4).read(&mut sys2).unwrap();
	assert_eq!(u32::from_le_bytes(sys2), ((OPCODE_SYS as u32) << (8 * 3)) + OPCODE_PUT_C as u32);

	let mut hlt = [0u8; 4];
	file.by_ref().take(4).read(&mut hlt).unwrap();
	assert_eq!(u32::from_le_bytes(hlt), (OPCODE_HLT as u32) << (8 * 3));

	fs::remove_file(path).unwrap();
}
