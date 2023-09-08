use windows::core::*;

#[allow(non_snake_case)]
pub fn String_to_PCSTR(s:String) -> PCSTR {
        return PCSTR(s.as_ptr() as *const u8);
}

#[allow(non_snake_case)]
pub fn slice_to_String(slice: &mut [u8], length: u32) -> String {
        match String::from_utf8(slice[0..(length as usize)].to_vec()) {
                Ok(string) => {string},
                Err(error) => {error.to_string()},
        }
}