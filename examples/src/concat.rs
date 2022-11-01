use libsql_bindgen::libsql_bindgen;

#[libsql_bindgen]
pub fn concat(s1: String, s2: String) -> String {
    let mut ret = s1.clone();
    ret += &s2;
    ret
}

fn main() {}