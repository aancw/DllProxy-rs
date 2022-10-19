use pelite::FileMap;
use pelite::pe64::{Pe, PeFile};
use pelite::pe64::exports::GetProcAddress;
fn main() {
	// Load the desired file into memory
	let file_map = FileMap::open("demo/Demo64.dll").unwrap();
    
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

	// Most convenient way to get the address of an export
	file.get_proc_address("ThrowException")?;

	// Access the export directory
	let exports = file.exports()?;

	// Print the export DLL name
	let dll_name = exports.dll_name()?;
	println!("dll_name: {}", dll_name);

	// To query the exports
	let by = exports.by()?;

	// For example: query an export by name
	by.name("?__autoclassinit2@Passwds@@QEAAX_K@Z")?;

	// For example: query an export by ordinal
	by.ordinal(6)?;

	// For example: iterate over all the exports.
	for result in by.iter() {
		if let Ok(export) = result {
			println!("export: {:?}", export);
		}
	}

	// For example: iterate over the named exports
	for result in by.iter_names() {
		if let (Ok(name), Ok(export)) = result {
			println!("export {}: {:?}", name, export);
		}
	}

	Ok(())
}