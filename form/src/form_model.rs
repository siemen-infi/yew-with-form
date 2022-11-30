use std::str::FromStr;

use validator::Validate;

pub trait FormValue {
    fn fields(&self, prefix: &str, fields: &mut Vec<String>) {
        fields.push(String::from(prefix));
    }

    fn value(&self, field_path: &str) -> String;

    fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), String>;
}

pub trait FormModel: FormValue + Validate + PartialEq + Clone + 'static {}

pub fn split_field_path(field_path: &str) -> (&str, &str) {
    if let Some(index) = field_path.find(".") {
        (&field_path[0..index], &field_path[index + 1..])
    } else {
        (field_path, "")
    }
}

impl<T: ToString + FromStr> FormValue for T {
    fn value(&self, field_path: &str) -> String {
        debug_assert!(field_path == "");

        self.to_string()
    }

    fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), String> {
        debug_assert!(field_path == "");

        if let Ok(v) = value.parse::<T>() {
            *self = v;
            Ok(())
        } else {
            Err(String::from("Could not parse value"))
        }
    }
}
