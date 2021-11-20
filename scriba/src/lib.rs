use std::fs::File;
use std::io::Write;

#[cfg(test)]
mod tests;

pub struct Magna {
	text: Vec<u32>,
	read_only: Vec<u8>,
	init_writable: Vec<u8>,
	global_uninit_size: usize
}

impl Magna {
	pub fn new() -> Magna {
		Magna { text: vec![], read_only: vec![], init_writable: vec![], global_uninit_size: 0 }
	}

	pub fn write_file(&self, path: &str) -> std::io::Result<()> {
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

