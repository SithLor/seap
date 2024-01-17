struct KeyValue {
    key: String,
    value: String,
}

fn keyvalue(key: &str, value: &str) -> KeyValue {
    KeyValue {
        key: key.to_string(),
        value: value.to_string(),
    }
}