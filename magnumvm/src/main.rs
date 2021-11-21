mod vm;

use std::{env, fs, io, io::Read};

use magnum_common::*;
use vm::VM;

#[cfg(test)]
mod tests;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 2 {
		panic!("Expect single arg!");
	}

	let mut vm = VM::load(&args[1]).unwrap();

	vm.run().unwrap();
}
