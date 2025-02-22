use serde_json::Value;

pub const REQUIRED_COOKIE_VALUE: &str = "required value";

pub fn keys() -> Value {
    serde_json::json!({
        "abc": 123,
    })
}
