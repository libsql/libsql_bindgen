#![no_std]

use core::ffi::{c_char, c_void, CStr};

static SQLITE_TEXT: u8 = 3;
static SQLITE_BLOB: u8 = 4;
static SQLITE_NULL: u8 = 5;

#[allow(dead_code)]
fn to_str<'a>(raw_ptr: *const c_char) -> &'a str {
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

fn to_slice<'a>(raw_ptr: *const c_void) -> &'a mut [u8] {
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

// libSQL: contains(base text, pattern text) -> int
#[no_mangle]
pub fn contains(base: *const c_char, pattern: *const c_char) -> i64 {
    let base = to_str(base);
    let pattern = to_str(pattern);
    base.contains(pattern) as i64
}

// libSQL: concat3(s1 text, s2 text, s3 text) -> text
#[no_mangle]
pub fn concat3(s1: *const c_char, s2: *const c_char, s3: *const c_char) -> *const c_char {
    let s1 = to_str(s1);
    let s2 = to_str(s2);
    let s3 = to_str(s3);

    let n = s1.len() + s2.len() + s3.len();
    let new_pages = (n + 65535) / 65536;
    let result_ptr = core::arch::wasm32::memory_grow(0, new_pages) * 65536;

    let result: &mut [u8] =
        unsafe { core::slice::from_raw_parts_mut(result_ptr as *mut u8, n + 2) };

    result[0] = SQLITE_TEXT;
    result[1..(s1.len() + 1)].copy_from_slice(s1.as_bytes());
    result[(s1.len() + 1)..(s1.len() + s2.len() + 1)].copy_from_slice(s2.as_bytes());
    result[(s1.len() + s2.len() + 1)..(n + 1)].copy_from_slice(s3.as_bytes());
    result[n + 1] = 0;
    result_ptr as *const c_char
}

// libSQL: fib(n int) -> int
#[no_mangle]
pub fn fib(n: i64) -> i64 {
    match n {
        0 | 1 => n,
        _ => fib(n - 1) + fib(n - 2),
    }
}

#[no_mangle]
pub fn reverse_blob(arg: *const c_void) -> *const c_void {
    to_slice(arg).reverse();
    arg
}

#[no_mangle]
pub fn get_null() -> *const c_char {
    let result_ptr = core::arch::wasm32::memory_grow(0, 1) * 65536;
    unsafe { *(result_ptr as *mut u8) = SQLITE_NULL };
    result_ptr as *const c_char
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
