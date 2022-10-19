// Copyright (c) 2022 Petruknisme
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

extern crate clap;
extern crate colored;

use clap::Parser;
use colored::Colorize;
use std::process;
use std::io::{self, Write};
use pelite::{FileMap, Wrap, PeFile};


#[derive(Parser)]
#[clap(name = "Spring4shell PoC")]
#[clap(author = "Petruknisme <me@petruknisme.com>")]
#[clap(version = "1.0")]
#[clap(about = "Spring 4 Shell Proof of Concept Script", long_about = None)]


struct Cli {
    /// Spring target url
    #[clap(short, long)]
    url: String,

    /// This mode for sending the payload
    #[clap(short, long)]
    deploy: bool,

    /// This mode for accessing payload with interactive shell
    #[clap(short, long)]
    shell: bool,
    
    /// Input command to run in exploit
    #[clap(short, long)]
    cmd: Option<String>,

}

fn main() {
	// Load the desired file into memory
	let file_map = FileMap::open(r"C:\Program Files\7-Zip\7z.dll").unwrap();
    println!("Searching exports function in DLL to hijack");
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

/*
fn check_buildtool_exist(){
	// cmd.exe /C '"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat" && cl.exe'
}*/