#[libsql_bindgen::libsql_bindgen]
pub fn concat(s1: String, s2: String) -> String {
    let mut s1 = s1;
    s1 += &s2;
    s1
}
