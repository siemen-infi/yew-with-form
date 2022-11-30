pub(crate) struct FormField {
    pub form_field_id: String,
    pub value: String,
    pub message: String,
    pub dirty: bool,
    pub valid: bool,
}

impl FormField {
    pub fn new(form_field_id: &str, value: &str) -> Self {
        Self {
            form_field_id: String::from(form_field_id),
            value: String::from(value),
            message: String::new(),
            dirty: false,
            valid: true,
        }
    }
}
