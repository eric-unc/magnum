use std::{fs, io};
use magnum_common::*;
use std::io::{BufRead, BufReader, Read};

pub struct VM {
	text: Vec<u32>,
	data: Vec<u8>,
	stack_start: usize,
	readable_start: usize,
	stack_base: usize,
	program_counter: usize
}

impl VM {
	pub fn load(path: &str) -> io::Result<VM> {
		let mut file = fs::File::open(path).unwrap();

		let mut file_sig = [0u8; 3];
		file.read_exact(&mut file_sig)?;

		if file_sig.ne(b"MVM") {
			panic!("File signature does not match! Found {:?}, expected {:?}.", file_sig, b"MVM");
		}

		let mut version = [0u8];
		file.read(&mut version)?;

		if version.ne(&[0]) {
			panic!("Version does not match! Found {:?}.", version);
		}

		let mut text_loc = [0u8; 8];
		file.read(&mut text_loc)?;

		let text_loc = usize::from_le_bytes(text_loc);
		if text_loc.ne(&0x35usize) {
			panic!("Text location does not match! Found {}.", text_loc);
		}

		let mut text_size = [0u8; 8];
		file.read(&mut text_size)?;
		let text_size = usize::from_le_bytes(text_size);

		if text_size % 4 != 0 {
			panic!("Instructions must be consistently 32-bit! Size is {}.", text_size);
		}

		let mut readonly_loc = [0u8; 8];
		file.read(&mut readonly_loc)?;
		let readonly_loc = usize::from_le_bytes(readonly_loc);

		let mut readonly_size = [0u8; 8];
		file.read(&mut readonly_size)?;
		let readonly_size = usize::from_le_bytes(readonly_size);

		let mut init_writable_loc = [0u8; 8];
		file.read(&mut init_writable_loc)?;
		let init_writable_loc = usize::from_le_bytes(init_writable_loc);

		let mut init_writable_size = [0u8; 8];
		file.read(&mut init_writable_size)?;
		let init_writable_size = usize::from_le_bytes(init_writable_size);

		let mut uninit_writable_size = [0u8; 8];
		file.read(&mut uninit_writable_size)?;
		let uninit_writable_size = usize::from_le_bytes(uninit_writable_size);

		let mut text = Vec::with_capacity(text_size);

		for _ in 0..(text_size / 4) {
			let mut instruction = [0u8; 4];
			file.read(&mut instruction)?;
			text.push(u32::from_le_bytes(instruction));
		}

		let mut data = Vec::with_capacity(readonly_size + init_writable_size + uninit_writable_size);

		for _ in 0..readonly_size {
			let mut byte = [0u8];
			file.read(&mut byte)?;
			data.push(u8::from_le_bytes(byte));
		}

		for _ in 0..init_writable_size {
			let mut byte = [0u8];
			file.read(&mut byte)?;
			data.push(u8::from_le_bytes(byte));
		}

		for _ in 0..uninit_writable_size {
			data.push(0);
		}

		let stack_start = data.len();
		let readable_start = readonly_size;

		Ok(VM { text, data, stack_start, readable_start, stack_base: 0, program_counter: 0})
	}

	pub fn run(&mut self) -> Result<(), String> {
		loop {
			if self.program_counter >= self.text.len() {
				return Ok(());
			}

			let inst = self.text[self.program_counter];
			let opcode = (inst >> (8 * 3)) as u8;

			match opcode {
				OPCODE_NOP => {}
				OPCODE_LOADI_B => {
					let im = (inst & 0xFF) as u8;
					self.data.push(im);
				}
				OPCODE_LOADI_2B => {
					let im1 = (inst & 0xFF) as u8;
					let im2 = ((inst & 0xFF00) >> 8) as u8;
					self.data.push(im2);
					self.data.push(im1);
				}
				OPCODE_LOADI_4B => {
					let im1 = (inst & 0xFF) as u8;
					let im2 = ((inst & 0xFF00) >> 8) as u8;
					let im3 = ((inst & 0xFF0000) >> (8 * 2)) as u8;
					self.data.push(0);
					self.data.push(im3);
					self.data.push(im2);
					self.data.push(im1);
				}
				OPCODE_LOADI_8B => {
					let im1 = (inst & 0xFF) as u8;
					let im2 = ((inst & 0xFF00) >> 8) as u8;
					let im3 = ((inst & 0xFF0000) >> (8 * 2)) as u8;
					self.data.push(0);
					self.data.push(0);
					self.data.push(0);
					self.data.push(0);
					self.data.push(0);
					self.data.push(im3);
					self.data.push(im2);
					self.data.push(im1);
				}
				OPCODE_LOAD => {
					let size = ((inst & 0xFF0000) >> (8 * 2)) as u8;
					let addr = (inst & 0xFFFF) as u16;

					// the casting is a real PITA
					let size = size as usize;
					let addr = addr as usize;

					if self.readable_start < addr as usize || addr + size - 1 < self.data.len() {
						panic!("Invalid address!");
					}

					for i in addr..(addr + size) {
						self.data.push(self.data[i]);
					}
				}
				OPCODE_STORE => {
					let size = ((inst & 0xFF0000) >> (8 * 2)) as u8;
					let addr = (inst & 0xFFFF) as u16;

					// the casting is a real PITA
					let size = size as usize;
					let addr = addr as usize;

					if self.readable_start < addr as usize || addr + size - 1 < self.data.len() {
						panic!("Invalid address!");
					}

					for i in addr..(addr + size) {
						self.data[i] = self.data.pop().unwrap();
					}
				}
				OPCODE_POP => {
					let size = (inst & 0xFF) as u8;

					for _ in 0..size {
						self.data.pop();
					}
				}
				OPCODE_FUNC_B => {
					let func = (inst & 0xFF) as u8;

					match func {
						OPCODE_ADD => {
							let a = self.data.pop().unwrap();
							let b = self.data.pop().unwrap();
							self.data.push(a + b);
						}
						OPCODE_SUB => {
							let a = self.data.pop().unwrap();
							let b = self.data.pop().unwrap();
							self.data.push(a - b);
						}
						OPCODE_MUL => {
							let a = self.data.pop().unwrap();
							let b = self.data.pop().unwrap();
							self.data.push(a * b);
						}
						OPCODE_DIV => {
							let a = self.data.pop().unwrap();
							let b = self.data.pop().unwrap();
							self.data.push(a / b);
						}
						_ => panic!("Unknown function {}!", func)
					}
				}
				OPCODE_SYS => {
					let call = (inst & 0xFF) as u8;

					match call {
						OPCODE_PUT_B => {
							let b = self.data.pop().unwrap();
							print!("{}", b);
						}
						OPCODE_PUT_C => {
							let c = self.data.pop().unwrap() as char;
							print!("{}", c);
						}
						_ => panic!("Unknown system call {}!", call)
					}
				}
				OPCODE_HLT => {
					std::process::exit(0);
				}
				_ => panic!("Unknown opcode {}!", opcode)
			}

			self.program_counter += 1;
		}
	}
}
