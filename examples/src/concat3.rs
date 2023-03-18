use libsql_bindgen::*;

#[libsql_bindgen::libsql_bindgen]
pub fn concat3(s1: String, s2: String, s3: String) -> String {
    let mut ret = s1.clone();
    ret += &s2;
    ret += &s3;
    ret
}

fn main() {}
