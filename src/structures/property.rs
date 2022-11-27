use std::collections::HashMap;
use std::fmt;

// TODO: Is there any way to do this dynamically without it being a pain in the ass?

#[derive(Clone)]
pub struct Property {
    pub numeric_value: Option<f64>,
    pub string_value: Option<String>,
    pub array_value: Option<Vec<Property>>,
    pub object_value: Option<HashMap<String, Box<Property>>>,
    pub bool_value: Option<bool>,
    pub is_null_value: bool,
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.numeric_value.is_some() {
            return write!(f, "{}", self.numeric_value.unwrap());
        }
        if self.string_value.is_some() {
            return write!(f, "{}", self.string_value.as_ref().unwrap());
        }
        if self.bool_value.is_some() {
            return write!(f, "{}", self.bool_value.unwrap());
        }

        if self.array_value.is_some() {
            return write!(
                f,
                "[{}]",
                self.array_value
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|prop| prop.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        }

        if self.object_value.is_some() {
            return write!(
                f,
                "{{ {} }}",
                self.object_value
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|(key, value)| format!("\"{}\": {}", key, value.to_string()))
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        }

        return if self.is_null_value {
            write!(f, "null")
        } else {
            write!(f, "<EMPTY>")
        }
    }
}