use std::path::Path;
use std::fs;
use std::io::Read;
use std::process::Command;
use scriba::{Function, Magna, SystemCall};
use scriba::Instruction;
use crate::VM;

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
		Instruction::FuncB(Function::Add),
		Instruction::Sys(SystemCall::PutB),
		Instruction::LoadIB('\n' as u8),
		Instruction::Sys(SystemCall::PutC)
	];

	for i in insts {
		magna.add_inst(i);
	}

	magna.write_file(path).unwrap();

	let output = Command::new("cargo")
		.args(["run", "--", path])
		.output()
		.unwrap();

	assert_eq!(output.stdout, b"2\n");

	fs::remove_file(path).unwrap();
}
