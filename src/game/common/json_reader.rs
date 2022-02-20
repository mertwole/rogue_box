pub struct JsonReader {}

impl JsonReader {
    pub fn read_string(obj: &serde_json::Value, name: &str, error: &mut bool) -> String {
        match obj {
            serde_json::Value::Object(obj) => match obj.get(name) {
                Some(serde_json::Value::String(st)) => st.clone(),
                _ => {
                    *error = true;
                    String::from("")
                }
            },
            _ => {
                *error = true;
                String::from("")
            }
        }
    }

    pub fn read_i32(obj: &serde_json::Value, name: &str, error: &mut bool) -> i32 {
        match obj {
            serde_json::Value::Object(obj) => match obj.get(name) {
                Some(serde_json::Value::Number(n)) => match n.as_i64() {
                    Some(n_i64) => n_i64 as i32,
                    _ => {
                        *error = true;
                        0
                    }
                },
                _ => {
                    *error = true;
                    0
                }
            },
            _ => {
                *error = true;
                0
            }
        }
    }

    pub fn read_f32(obj: &serde_json::Value, name: &str, error: &mut bool) -> f32 {
        match obj {
            serde_json::Value::Object(obj) => match obj.get(name) {
                Some(serde_json::Value::Number(n)) => match n.as_f64() {
                    Some(n_f64) => n_f64 as f32,
                    _ => {
                        *error = true;
                        0.0
                    }
                },
                _ => {
                    *error = true;
                    0.0
                }
            },
            _ => {
                *error = true;
                0.0
            }
        }
    }

    pub fn read_vec(
        obj: &serde_json::Value,
        name: &str,
        error: &mut bool,
    ) -> Vec<serde_json::Value> {
        match obj {
            serde_json::Value::Object(obj) => match obj.get(name) {
                Some(serde_json::Value::Array(arr)) => arr.to_vec(),
                _ => {
                    *error = true;
                    Vec::new()
                }
            },
            _ => {
                *error = true;
                Vec::new()
            }
        }
    }

    pub fn read_obj(obj: &serde_json::Value, name: &str, error: &mut bool) -> serde_json::Value {
        match obj {
            serde_json::Value::Object(obj) => {
                let sub_obj = obj.get(name);
                match sub_obj {
                    Some(serde_json::Value::Object(_)) => sub_obj.unwrap().clone(),
                    _ => {
                        *error = true;
                        serde_json::Value::Object(serde_json::Map::new())
                    }
                }
            }
            _ => {
                *error = true;
                serde_json::Value::Object(serde_json::Map::new())
            }
        }
    }
}
