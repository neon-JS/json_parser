use std::collections::HashMap;

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

pub trait ToProperty {
    fn convert_to_property(&self) -> Property;
}