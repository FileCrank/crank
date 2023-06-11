// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{io::{BufRead, Write}, path::Path};

use convert::{format::Format, error::ConversionResult, format::{FORMATS}};
use std::ops::Deref;


/*
#[tauri::command]
fn convert(
    from: Format,
    to: Format,
    source_file: String,
    dest: String,
) {
}
 */


#[tauri::command]
fn get_formats() -> &'static Vec<&'static Format> {
    FORMATS.deref()
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_formats])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
