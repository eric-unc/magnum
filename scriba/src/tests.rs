use std::path::Path;
use std::fs;
use std::io::Read;
use crate::Magna;
use crate::Instruction;

#[test]
fn simple_hlt() {
	let path = "simple-hlt.magna";

	if Path::new(path).exists() {
		fs::remove_file(path).unwrap();
	}

	let mut magna = Magna::new();
	magna.add_inst(Instruction::Hlt);
	magna.write_file(path).unwrap();

	let mut file = fs::File::open(path).unwrap();

	let mut file_sig = [0u8; 3];
	file.by_ref().take(3).read(&mut file_sig).unwrap();
	assert_eq!(file_sig, "MVM".as_bytes());

	let mut version = [0u8];
	file.by_ref().take(3).read(&mut version).unwrap();
	assert_eq!(version, [0u8]);

	let mut text_start = [0u8; 8];
	file.by_ref().take(8).read(&mut text_start).unwrap();
	assert_eq!(text_start, [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0x35]);

	fs::remove_file(path).unwrap();
}


