use libsql_bindgen::*;
// use serde
fn main() {}

#[libsql_bindgen::libsql_bindgen]
pub fn get_exchange_rate(from: String, to: String) -> f32 {
    let rate = get_online_exchange_rate(&from, &to)
        .map(|api| api.data.rate)
        .unwrap_or(0.0);
    rate
}

#[derive(serde::Deserialize)]
struct APIResult {
    #[serde(default)]
    data: Rate,
}
#[derive(serde::Deserialize, Default)]
struct Rate {
    rate: f32,
}

fn get_online_exchange_rate(from: &str, to: &str) -> Option<APIResult> {
    let mut writer = Vec::new();
    let uri = format!(
        "https://api.it120.cc/gooking/forex/rate?fromCode={}&toCode={}",
        to, from,
    );
    let _ = http_req::request::get(uri, &mut writer).ok()?;
    let r = serde_json::from_slice::<APIResult>(&writer).ok()?;
    Some(r)
}
