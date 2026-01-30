use serde_json::json;

pub fn generate_memo(action: &str, data: &serde_json::Value) -> String {
    let memo = json!({
        "protocol": "launchpad-skill",
        "action": action,
        "data": data,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    memo.to_string()
}
