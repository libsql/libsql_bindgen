use core::ffi::{c_char, c_void, CStr};

pub use libsql_bindgen_macros::libsql_bindgen;

static SQLITE_TEXT: u8 = 3;
static SQLITE_BLOB: u8 = 4;
static SQLITE_NULL: u8 = 5;

// Type translation from Wasm pointers to strings, blobs and nulls
pub trait FromLibSQL {
    fn from_libsql_type(wasm_ptr: i32) -> Self;
}

// Type translation from strings, blobs and nulls to Wasm pointers
pub trait IntoLibSQL {
    fn into_libsql_type(self) -> i32;
}

impl FromLibSQL for &str {
    fn from_libsql_type(wasm_ptr: i32) -> Self {
        let raw_ptr = wasm_ptr as *const c_char;
        unsafe {
            if *raw_ptr != SQLITE_TEXT as c_char {
                "???"
            } else {
                match CStr::from_ptr(raw_ptr.offset(1)).to_str() {
                    Ok(s) => s,
                    Err(_) => "!!!",
                }
            }
        }
    }
}

impl IntoLibSQL for &str {
    fn into_libsql_type(self) -> i32 {
        let mut mem: Vec<u8> = vec![0; self.len() + 2];
        mem[0] = SQLITE_TEXT;
        mem[1..=self.len()].copy_from_slice(self.as_bytes());
        mem[self.len() + 1] = 0;
        let ptr = mem.as_ptr() as i32;
        std::mem::forget(mem);
        ptr
    }
}

impl FromLibSQL for String {
    fn from_libsql_type(wasm_ptr: i32) -> Self {
        <&str>::from_libsql_type(wasm_ptr).to_owned()
    }
}

impl IntoLibSQL for String {
    fn into_libsql_type(self) -> i32 {
        self.as_str().into_libsql_type()
    }
}

impl FromLibSQL for &mut [u8] {
    fn from_libsql_type(wasm_ptr: i32) -> Self {
        let raw_ptr = wasm_ptr as *const c_void;
        if unsafe { *(raw_ptr as *const c_char) != SQLITE_BLOB as c_char } {
            &mut []
        } else {
            let size = unsafe {
                u32::from_be_bytes(
                    core::slice::from_raw_parts(raw_ptr.offset(1) as *const u8, 4)
                        .try_into()
                        .unwrap_or([0_u8, 0_u8, 0_u8, 0_u8]),
                )
            };
            unsafe { core::slice::from_raw_parts_mut(raw_ptr.offset(5) as *mut u8, size as usize) }
        }
    }
}

impl<T: FromLibSQL> FromLibSQL for Option<T> {
    fn from_libsql_type(wasm_ptr: i32) -> Self {
        let raw_ptr = wasm_ptr as *const c_char;
        unsafe {
            if *raw_ptr == SQLITE_NULL as c_char {
                None
            } else {
                Some(<T>::from_libsql_type(wasm_ptr))
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn libsql_malloc(size: usize) -> usize {
    let buffer = Vec::<u8>::with_capacity(size);
    let ptr = Vec::leak(buffer);
    ptr.as_ptr() as usize
}
