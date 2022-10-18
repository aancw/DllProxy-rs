use pelite::FileMap;
use pelite::pe32::{Pe, PeFile};

fn main() {
	// Load the desired file into memory
	let file_map = FileMap::open("demo/Demo.dll").unwrap();
    
	// Process the image file
	dll_deps(file_map.as_ref()).unwrap();
}

pub fn dll_deps(image: &[u8]) -> pelite::Result<()> {
	// Interpret the bytes as a PE32+ executable
	let file = PeFile::from_bytes(image)?;

	// Let's read the DLL dependencies
	let imports = file.imports()?;
	for desc in imports {
		// Get the DLL name being imported from
		let dll_name = desc.dll_name()?;
		// Get the number of imports for this dll
		let iat = desc.iat()?;
		println!("imported {} functions from {}", iat.len(), dll_name);
	}

	Ok(())
}