// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{io::{BufReader}, fs::File};

use convert::{format::Format, error::{ConversionResult}, format::{FORMATS}, convert};
use std::ops::Deref;


#[tauri::command]
fn convert_file(
    from: Format,
    to: Format,
    source_file: String
) -> ConversionResult<Vec<u8>> {
    let src = File::open(source_file)?;
    let mut src_reader = BufReader::new(src);
    let mut dest: Vec<u8> = Vec::new();

    convert(&from, &to, &mut src_reader, &mut dest)?;
    
    Ok(dest)
}



#[tauri::command]
fn get_formats() -> &'static Vec<&'static Format<'static>> {
    FORMATS.deref()
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![convert_file, get_formats])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
