use serde_json::Value;

pub const REQUIRED_COOKIE_VALUE: &str = "317";

pub fn keys() -> Value {
    serde_json::json!({
        "message": "Hello World!",
        "status": "success",
        "code": 200
    })
}
