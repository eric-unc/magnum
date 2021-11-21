use std::fs::File;
use std::io::Write;
use std::io;

use magnum_common::*;

#[cfg(test)]
mod tests;

pub struct Magna {
	text: Vec<u32>,
	read_only: Vec<u8>,
	init_writable: Vec<u8>,
	global_uninit_size: usize
}

#[derive(Clone, Copy)]
pub enum Instruction {
	Nop,
	LoadIB(u8),
	LoadI2B(u16),
	LoadI4B(u32),
	LoadI8B(u32),
	Load(u8, u16),
	Loads(u8, u16),
	Store(u8, u16),
	Pop(u8),
	FuncB(Function),
	Sys(SystemCall),
	Hlt
}

use Instruction::*;

#[derive(Clone, Copy)]
pub enum Function {
	Add,
	Sub,
	Mul,
	Div
}

use Function::*;

#[derive(Clone, Copy)]
pub enum SystemCall {
	PutB,
	PutC
}

use SystemCall::*;

impl Magna {
	pub fn new() -> Magna {
		Magna { text: vec![], read_only: vec![], init_writable: vec![], global_uninit_size: 0 }
	}

	pub fn add_inst(&mut self, inst: Instruction) {
		/// "Empty" function, just to make our lives a bit easier.
		/// Returns an empty instruction with just the opcode.
		fn e(opcode: u8) -> u32 {
			(opcode as u32) << (8 * 3)
		}

		fn func_opcode(func: Function) -> u32 {
			(match func {
				Add => OPCODE_ADD,
				Sub => OPCODE_SUB,
				Mul => OPCODE_MUL,
				Div => OPCODE_DIV
			}) as u32
		}

		fn call_opcode(call: SystemCall) -> u32 {
			(match call {
				PutB => OPCODE_PUT_B,
				PutC => OPCODE_PUT_C
			}) as u32
		}

		let inst: u32 = match inst {
			Nop => e(OPCODE_NOP),
			LoadIB(im)=> e(OPCODE_LOADI_B) + im as u32,
			LoadI2B(im)=> e(OPCODE_LOADI_2B) + im as u32,
			LoadI4B(im)=> e(OPCODE_LOADI_4B) + (0xFFFFFF & im as u32),
			LoadI8B(im)=> e(OPCODE_LOADI_8B) + (0xFFFFFF & im as u32),
			Load(size, addr) => e(OPCODE_LOAD) + ((size as u32) << (8 * 1)) + addr as u32,
			Loads(size, offset) => e(OPCODE_LOADS) + ((size as u32) << (8 * 1)) + offset as u32,
			Store(size, addr) => e(OPCODE_STORE) + ((size as u32) << (8 * 1)) + addr as u32,
			Pop(size) => e(OPCODE_POP) + size as u32,
			FuncB(func) => e(OPCODE_FUNC_B) + func_opcode(func),
			Sys(call) => e(OPCODE_SYS) + call_opcode(call),
			Hlt => e(OPCODE_HLT)
		};

		self.add_inst_from_u32(inst);
	}

	pub fn add_inst_from_u32(&mut self, inst: u32) {
		self.text.push(inst);
	}

	pub fn write_file(&self, path: &str) -> io::Result<()> {
		let mut file = File::create(path)?;

		// Header
		{
			// File signature
			file.write_all(b"MVM")?;

			// Target version
			file.write_all(&[0])?;

			// Location of text section
			let offset_text: usize = 0x35;
			file.write_all(&offset_text.to_le_bytes())?;

			// Size of text section
			file.write_all(&self.text.len().to_le_bytes())?;

			// Location of read-only section
			let offset_read_only = offset_text + self.text.len();
			file.write_all(&offset_read_only.to_le_bytes())?;

			// Size of read-only section
			file.write_all(&self.read_only.len().to_le_bytes())?;

			// Location of initialized writable section
			let offset_init_writable = offset_read_only + self.read_only.len();
			file.write_all(&offset_init_writable.to_le_bytes())?;

			// Size of initialized writable section
			file.write_all(&self.init_writable.len().to_le_bytes())?;

			// Size of uninitialized writable memory
			file.write_all(&self.global_uninit_size.to_le_bytes())?;
		}

		// Sections
		{
			// I feel like this isn't the best way to do this but fuck it.
			// Text
			for inst in self.text.iter() {
				file.write_all(&inst.to_le_bytes())?;
			}

			// Read-only
			for byte in self.read_only.iter() {
				file.write_all(&byte.to_le_bytes())?;
			}

			// Initialized writable
			for byte in self.init_writable.iter() {
				file.write_all(&byte.to_le_bytes())?;
			}
		}

		Ok(())
	}
}
