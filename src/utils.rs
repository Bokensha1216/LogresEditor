use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::*;

pub fn hex_string_to_bytes(hex_string: &str) -> Vec<u8> {
        let hex_string = hex_string.trim_start_matches("0x"); // "0x" プレフィックスを取り除く
        let mut bytes = Vec::new();
    
        for i in (0..hex_string.len()).step_by(2) {
            let byte_str = &hex_string[i..(i + 2)];
            let byte = u8::from_str_radix(byte_str, 16).unwrap();
            bytes.push(byte);
        }
    
        return bytes;
    }

pub fn message_box(text: String, title: String) {
        unsafe { 
                MessageBoxA(None, String_to_PCSTR(text), String_to_PCSTR(title), MB_OK);
        }
}

#[allow(non_snake_case)]
pub fn String_to_PCSTR(mut s:String) -> PCSTR {
        s.push('\0');
        return PCSTR(s.as_ptr() as *const u8);
}

#[allow(non_snake_case)]
pub fn slice_to_String(slice: &mut [u8], length: u32) -> String {
        match String::from_utf8(slice[0..(length as usize)].to_vec()) {
                Ok(string) => {string},
                Err(error) => {error.to_string()},
        }
}