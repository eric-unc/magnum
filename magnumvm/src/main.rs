use std::{env, fs, io, io::Read};

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 2 {
		panic!("Expect single arg!");
	}
}

struct VM {
	text: Vec<u32>,
	data: Vec<u8>,
	stack_base: usize
}

impl VM {
	fn load(path: &str) -> io::Result<VM> {
		//let vm = VM { text: vec![], data: vec![], stack_base: 0};

		let mut file = fs::File::open(path)?;

		let mut file_sig = [0u8; 3];
		file.by_ref().take(3).read(&mut file_sig)?;

		if file_sig.ne("MVM".as_bytes()) {
			panic!("File signature does not match!");
		}

		let mut version = [0u8];
		file.by_ref().take(3).read(&mut version)?;

		if version.ne(&[0]) {
			panic!("Version does not match!");
		}

		let mut text_loc = [0u8; 8];
		file.by_ref().take(8).read(&mut text_loc)?;

		if usize::from_le_bytes(text_loc).ne(&0x35usize) {
			panic!("Version does not match!");
		}

		let mut text_size = [0u8; 8];
		file.by_ref().take(8).read(&mut text_size)?;
		let text_size = usize::from_le_bytes(text_size);

		if text_size % 4 != 0 {
			panic!("Instructions must be consistently 32-bit!");
		}

		//let mut text = Vec::with_capacity(text_size);

		let mut readonly_loc = [0u8; 8];
		file.by_ref().take(8).read(&mut readonly_loc)?;
		let readonly_loc = usize::from_le_bytes(readonly_loc);

		let mut readonly_size = [0u8; 8];
		file.by_ref().take(8).read(&mut readonly_size)?;
		let readonly_size = usize::from_le_bytes(readonly_size);

		let mut init_writable_loc = [0u8; 8];
		file.by_ref().take(8).read(&mut init_writable_loc)?;
		let init_writable_loc = usize::from_le_bytes(init_writable_loc);

		let mut init_writable_size = [0u8; 8];
		file.by_ref().take(8).read(&mut init_writable_size)?;
		let init_writable_size = usize::from_le_bytes(init_writable_size);

		let mut uninit_writable_size = [0u8; 8];
		file.by_ref().take(8).read(&mut uninit_writable_size)?;
		let uninit_writable_size = usize::from_le_bytes(uninit_writable_size);

		let mut text = Vec::with_capacity(text_size);

		for _ in 0..(text_size / 4) {
			let mut instruction = [0u8, 4];
			file.by_ref().take(4).read(&mut instruction)?;
			text.push(u32::from_le_bytes(instruction));
		}

		let mut data = Vec::with_capacity(readonly_size + init_writable_size + uninit_writable_size);

		for _ in 0..readonly_size {
			let mut byte = [0u8, 1];
			file.by_ref().take(1).read(&mut byte)?;
			data.push(u8::from_le_bytes(byte));
		}

		for _ in 0..init_writable_size {
			let mut byte = [0u8, 1];
			file.by_ref().take(1).read(&mut byte)?;
			data.push(u8::from_le_bytes(byte));
		}

		for _ in 0..uninit_writable_size {
			data.push(0);
		}


		Ok(VM { text, data: vec![], stack_base: 0})
	}
}
