use std::{
    ffi::{CStr, CString},
    ptr::{null, null_mut},
};

pub fn make_frag()->CString{
    let g = CString::new(include_str!(".frag")).unwrap();
    //let k = g.to_string_lossy().into_owned();
    let original_string = "Hello, Rust! Rust is awesome.";

    // Remove "Rust" from the string
    let modified_string = original_string.replace("Rust", "");

    println!("{}", modified_string);
    
    g
}