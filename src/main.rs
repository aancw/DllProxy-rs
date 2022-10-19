use std::process;
use std::io::{self, Write};
use pelite::{FileMap, Wrap, PeFile};

fn main() {
	// Load the desired file into memory
	let file_map = FileMap::open(r"C:\Program Files\7-Zip\7z.dll").unwrap();
    
	// Process the image file
	match PeFile::from_bytes(&file_map) {
		Ok(Wrap::T32(file)) => dump_export32(file),
		Ok(Wrap::T64(file)) => dump_export64(file),
		Err(err) => abort(&format!("{}", err)),
	}
}

fn dump_export64(file: pelite::pe64::PeFile) {
	use pelite::pe64::Pe;

	let exports = file.exports().unwrap();

	let dll_name = exports.dll_name().unwrap();
	println!("dll_name: {}", dll_name);
	let by = exports.by().unwrap();

	for result in by.iter_names() {
		if let (Ok(name), Ok(export)) = result {
			println!("export {}: {:?}", name, export);
		}
	}
}

fn dump_export32(file: pelite::pe32::PeFile) {
	use pelite::pe32::Pe;
	
	let exports = file.exports().unwrap();

	let dll_name = exports.dll_name().unwrap();
	println!("dll_name: {}", dll_name);
	let by = exports.by().unwrap();

	for result in by.iter_names() {
		if let (Ok(name), Ok(export)) = result {
			println!("export {}: {:?}", name, export);
		}
	}
}

fn abort(message: &str) -> ! {
	{
		let stderr = io::stderr();
		let mut stderr = stderr.lock();
		let _ = stderr.write(b"dump: ");
		let _ = stderr.write(message.as_bytes());
		let _ = stderr.write(b".\n");
		let _ = stderr.flush();
	}
	process::exit(1);
}