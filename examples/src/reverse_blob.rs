use libsql_bindgen::*;

#[libsql_bindgen::libsql_bindgen]
pub fn reverse_blob(blob: &mut [u8]) -> Vec<u8> {
    let mut r = blob.to_vec();
    r.reverse();
    r
}

fn main() {}
