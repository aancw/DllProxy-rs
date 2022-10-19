use pelite::FileMap;
use pelite::pe64::PeFile as PeFile64;
use pelite::pe64::Pe as Pe64;
use pelite::pe32::PeFile as PeFile32;
use pelite::pe32::Pe as Pe32;
use pelite::Error as PeError;

fn main() {
	// Load the desired file into memory
	let file_map = FileMap::open("demo/Demo64.dll").unwrap();
    
	// Process the image file
	get_dll(file_map.as_ref()).unwrap();
}

fn get_dll(image: &[u8]) -> pelite::Result<()> {

	static mut export_func = None;
	match PeFile64::from_bytes(image) {
        Ok(pe) => {
			// Let's read the DLL dependencies
			let imports = pe.imports()?;
			for desc in imports {
				// Get the DLL name being imported from
				let dll_name = desc.dll_name()?;
				// Get the number of imports for this dll
				let iat = desc.iat()?;
				println!("imported {} functions from {}", iat.len(), dll_name);
			}
        },
        Err(e) => {
            if let PeError::PeMagic = e {
                let pe = PeFile32::from_bytes(image)?;
				// Let's read the DLL dependencies
				let imports = pe.imports()?;
				for desc in imports {
					// Get the DLL name being imported from
					let dll_name = desc.dll_name()?;
					// Get the number of imports for this dll
					let iat = desc.iat()?;
					println!("imported {} functions from {}", iat.len(), dll_name);
				}
            } 
        }
    };
	Ok(())
}
