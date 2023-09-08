mod process_editor;
use process_editor::*;
extern crate utils;
use utils::*;


fn main() {
    let window_name:String = "logres_andapp".to_string();
    let pid: u32 = get_process_id_from_window_name(window_name);
    println!("pid: {}", pid);

    let process_handle: windows::Win32::Foundation::HANDLE = open_process(pid);
    let modules: Vec<windows::Win32::Foundation::HMODULE> = get_process_modules(process_handle);
    let module_hashmap: std::collections::HashMap<String, windows::Win32::Foundation::HMODULE> = get_modules_names_hashmap(process_handle, modules);
    
    let base_address:usize = module_hashmap["logres_andapp.exe"].0 as usize;
    let rva: usize = 0x387E97;
    let data:&str = "0xE91E020000";
    let target_address: usize = base_address + rva;

    println!("target_address: 0x{:X}", target_address);
    println!("overwrite code: {}", data.to_string());

    write_process_memory(process_handle, target_address, hex_string_to_bytes(data));
    
    message_box("success".to_string(), "LogresEditor".to_string());
}
