mod process_editor;
use process_editor::*;

fn main() {
    let window_name:String = "logres_andapp".to_string();
    let pid: u32 = get_process_id_from_window_name(window_name);
    println!("pid: {}", pid);

    let process_handle = open_process(pid);
    let modules = get_process_modules(process_handle);
    let module_hashmap: std::collections::HashMap<String, windows::Win32::Foundation::HMODULE> = get_modules_names_hashmap(process_handle, modules);
    println!("{:?}", module_hashmap);

    // let data:&[u8] = &[0x12, 0x34];
    // let address: usize = 0x00400000;
    // write_process_memory(process_handle, address, data);
    println!("Hello, world!");
}
