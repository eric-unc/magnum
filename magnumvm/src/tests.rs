use std::path::Path;
use std::fs;
use std::io::Read;
use std::process::Command;
use scriba::{Function, Magna, SystemCall};
use scriba::Instruction;
use crate::VM;

#[test]
fn nop() {
	let path = "nop.magna";

	// Required if test fails, then the file won't be deleted at the end.
	if Path::new(path).exists() {
		fs::remove_file(path).unwrap();
	}

	let mut magna = Magna::new();

	let insts = [
		Instruction::Nop
	];

	for i in insts {
		magna.add_inst(i);
	}

	magna.write_file(path).unwrap();

	let output = Command::new("cargo")
		.args(["run", "--", path])
		.output()
		.unwrap();

	assert_eq!(output.stdout, b"");

	fs::remove_file(path).unwrap();
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

#[test]
fn complex_expr() {
	let path = "complex-expr.magna";

	// Required if test fails, then the file won't be deleted at the end.
	if Path::new(path).exists() {
		fs::remove_file(path).unwrap();
	}

	let mut magna = Magna::new();

	let insts = [
		Instruction::LoadIB(5),
		Instruction::LoadIB(5),
		Instruction::FuncB(Function::Add),
		Instruction::LoadIB(2),
		Instruction::FuncB(Function::Div),
		Instruction::LoadIB(3),
		Instruction::FuncB(Function::Mul),
		Instruction::LoadIB(3),
		Instruction::FuncB(Function::Sub),
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

	assert_eq!(output.stdout, b"12\n");

	fs::remove_file(path).unwrap();
}

#[test]
fn hello_world() {
	let path = "hello-world.magna";

	// Required if test fails, then the file won't be deleted at the end.
	if Path::new(path).exists() {
		fs::remove_file(path).unwrap();
	}

	let mut magna = Magna::new();

	let insts = [
		Instruction::LoadIB('H' as u8),
		Instruction::Sys(SystemCall::PutC),
		Instruction::LoadIB('e' as u8),
		Instruction::Sys(SystemCall::PutC),
		Instruction::LoadIB('l' as u8),
		Instruction::Sys(SystemCall::PutC),
		Instruction::LoadIB('l' as u8),
		Instruction::Sys(SystemCall::PutC),
		Instruction::LoadIB('o' as u8),
		Instruction::Sys(SystemCall::PutC),
		Instruction::LoadIB(' ' as u8),
		Instruction::Sys(SystemCall::PutC),
		Instruction::LoadIB('W' as u8),
		Instruction::Sys(SystemCall::PutC),
		Instruction::LoadIB('o' as u8),
		Instruction::Sys(SystemCall::PutC),
		Instruction::LoadIB('r' as u8),
		Instruction::Sys(SystemCall::PutC),
		Instruction::LoadIB('l' as u8),
		Instruction::Sys(SystemCall::PutC),
		Instruction::LoadIB('d' as u8),
		Instruction::Sys(SystemCall::PutC),
		Instruction::LoadIB('!' as u8),
		Instruction::Sys(SystemCall::PutC),
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

	assert_eq!(output.stdout, b"Hello World!\n");

	fs::remove_file(path).unwrap();
}
