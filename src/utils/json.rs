use serde::Serialize;

pub fn json_to_object<T: serde::de::DeserializeOwned>(json: &str) -> T {
    serde_json::from_str(json).unwrap()
}

/**
 * Convert a struct to a serde_json::Value
 */
pub fn object_to_json<T: Serialize>(t: &T) -> serde_json::Value {
    serde_json::to_value(t).unwrap()
}
