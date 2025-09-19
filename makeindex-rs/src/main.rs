mod genind;
mod mkind;
mod qsort;
mod scanid;
mod scanst;
mod sortid;

use std::env;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len() as i32;

    // Convert Rust strings to C-compatible strings
    let c_strings: Vec<CString> = args
        .iter()
        .map(|arg| CString::new(arg.as_str()).unwrap())
        .collect();

    // Create array of raw pointers
    let mut argv: Vec<*mut c_char> = c_strings
        .iter()
        .map(|s| s.as_ptr() as *mut c_char)
        .collect();

    // Add null terminator
    argv.push(ptr::null_mut());

    // Call the C main function from mkind module
    unsafe {
        let exit_code = mkind::makeindex_main(argc, argv.as_mut_ptr());
        std::process::exit(exit_code);
    }
}
