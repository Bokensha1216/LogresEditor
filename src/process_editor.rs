use std::collections::HashMap;
use std::ffi::c_void;

use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::*;
use windows::Win32::System::Diagnostics::Debug::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::System::Memory::*;
use windows::Win32::System::ProcessStatus::*;

mod utils;
use utils::*;

pub fn write_process_memory(process_handle: HANDLE, address: usize, data_to_write: &[u8]) {
        unsafe {
                change_protect_to_readwrite(process_handle, address);

                WriteProcessMemory(
                        process_handle,
                        address as *const c_void,
                        data_to_write.as_ptr() as *const c_void,
                        data_to_write.len(),
                        None
                ).unwrap();
        }
}

fn change_protect_to_readwrite(process_handle:HANDLE , address: usize){
	// 保護状態変更
	let mut mbi: MEMORY_BASIC_INFORMATION = Default::default();
	unsafe {
		VirtualQueryEx(process_handle, Some(address as *const c_void) , &mut mbi, std::mem::size_of::<MEMORY_BASIC_INFORMATION>());
		VirtualProtectEx(process_handle, mbi.BaseAddress, mbi.RegionSize, PAGE_READWRITE, &mut mbi.Protect).unwrap();
	}
}

pub fn open_process(pid: u32) -> windows::Win32::Foundation::HANDLE {
        unsafe {
                return OpenProcess(PROCESS_ALL_ACCESS, false, pid).unwrap();
        }
}

pub fn get_process_id_from_window_name(window_name: String) -> u32 {
        unsafe {
                let window_handle = FindWindowA(None, String_to_PCSTR(window_name));
                let mut pid:u32 = Default::default();
                GetWindowThreadProcessId(window_handle, Some(&mut pid));
                GetLastError().unwrap();
                return pid;
        }
}

pub fn get_process_modules(process_handle: HANDLE) -> Vec<windows::Win32::Foundation::HMODULE> {
        let mut process_modules: [HMODULE; 100] = [Default::default(); 100];

        unsafe {
                let mut needed:u32 = Default::default();
                let cb:u32 = (process_modules.len() * std::mem::size_of::<HMODULE>()) as u32;

                EnumProcessModules(
                        process_handle,
                        process_modules.as_mut_ptr(),
                        cb,
                        &mut needed,
                ).unwrap();

                let module_count = needed as usize / std::mem::size_of::<HMODULE>();

                return process_modules[0..module_count].to_vec();
        }
}

pub fn get_modules_names_hashmap(process_handle: HANDLE, modules: Vec<windows::Win32::Foundation::HMODULE>)  -> HashMap<String, windows::Win32::Foundation::HMODULE> {
        let module_names:Vec<String> = modules.iter()
                                                .map(|module| get_module_name(process_handle, *module))
                                                .collect();

        let process_modules_names: HashMap<String, HMODULE> = module_names.iter().zip(modules.iter()).map(|(s, h)| (s.clone(), *h)).collect();

        return process_modules_names
}

pub fn get_module_name(process_handle: HANDLE, module_handle: HMODULE) -> String {
        let module_name:&mut [u8] = &mut [Default::default(); 200];

        unsafe {
                let name_length = GetModuleBaseNameA(
                                process_handle,
                                module_handle,
                                module_name
                );

                GetLastError().unwrap();
                return slice_to_String(module_name, name_length);
        }
}