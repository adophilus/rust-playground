use serde::Serialize;
use std::string::ToString;

#[derive(Serialize)]
pub enum Human {
    #[serde(rename = "MAN")]
    Man,
    #[serde(rename = "WOMAN")]
    Woman,
}

impl ToString for Human {
    fn to_string(&self) -> String {
        match serde_json::to_value(self).unwrap() {
            serde_json::Value::String(string) => string,
            _ => unreachable!(),
        }
    }
}
