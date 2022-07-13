use serde_json::{json, Value};

pub fn sku(sku: &str) -> Value {
    json!({
        "query": {
            "term": {
                "sku": {
                    "value": sku
                }
            }
        }
    })
}

pub fn default() -> Value {
    json!({
        "query": {
            "match_all": {}
        }
    })
}