use std::collections::HashMap;
use std::fmt;

pub struct Property {
    pub number: Option<f64>,
    pub string: Option<String>,
    pub array: Option<Vec<Property>>,
    pub object: Option<HashMap<String, Box<Property>>>,
    pub bool: Option<bool>,
    pub is_null: bool,
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_null {
            return write!(f, "null");
        }

        if let Some(bool) = self.bool {
            return write!(f, "{}", bool);
        }

        if let Some(number) = self.number {
            return write!(f, "{}", number);
        }

        if let Some(string) = &self.string {
            return write!(f, "{}", string);
        }

        if let Some(array) = &self.array {
            let mut properties = array
                .iter()
                .map(|prop| prop.to_string())
                .collect::<Vec<String>>();

            properties.sort();

            return write!(f, "[{}]", properties.join(", "));
        }

        if let Some(object) = &self.object {
            let mut properties = object
                .iter()
                .map(|(key, value)| format!("\"{}\": {}", key, value))
                .collect::<Vec<String>>();

            properties.sort();

            return write!(f, "{{{}}}", properties.join(", "));
        }

        return write!(f, "<EMPTY>");
    }
}