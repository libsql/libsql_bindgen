use libsql_bindgen::*;

#[libsql_bindgen::libsql_bindgen]
pub fn contains(s1: String, s2: String) -> bool {
    s1.contains(&s2)
}

fn main() {}
