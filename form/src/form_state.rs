use validator::{ValidationErrors, ValidationErrorsKind};

use super::{form_field::FormField, form_model::FormModel};

pub(crate) struct FormState<T: FormModel> {
    pub(crate) model: T,
    fields: Vec<FormField>,
}

impl<T: FormModel> FormState<T> {
    pub fn new(model: T) -> FormState<T> {
        let mut fields = vec![];

        model.fields("", &mut fields);

        Self {
            model: model.clone(),
            fields: fields
                .iter()
                .map(|f| FormField::new(f, &model.value(f)))
                .collect(),
        }
    }

    pub fn model(&self) -> &T {
        &self.model
    }

    pub fn model_mut(&mut self) -> &mut T {
        &mut self.model
    }

    pub(crate) fn field(&self, form_field_id: &str) -> &FormField {
        self.fields
            .iter()
            .find(|&f| f.form_field_id == form_field_id)
            .expect(&format!("Field {} does not exist", form_field_id))
    }

    pub(crate) fn field_mut(&mut self, form_field_id: &str) -> &mut FormField {
        self.fields
            .iter_mut()
            .find(|f| f.form_field_id == form_field_id)
            .expect(&format!("Field {} does not exist", form_field_id))
    }

    pub fn field_value(&self, form_field_id: &str) -> &str {
        let field = self.field(form_field_id);

        &field.value
    }

    pub fn set_field_value(&mut self, field_path: &str, field_value: &str) {
        if self.field_value(field_path) != field_value {
            let result = self.model.set_value(field_path, field_value);

            let field = self.field_mut(field_path);
            field.value = field_value.into();
            field.dirty = true;

            match result {
                Ok(_) => {
                    field.valid = true;
                    field.message = String::new();
                }
                Err(e) => {
                    field.valid = false;
                    field.message = String::from(e);
                }
            }
        }
    }

    pub fn valid(&self) -> bool {
        self.fields.iter().all(|f| f.valid)
    }

    pub fn field_valid(&self, field_path: &str) -> bool {
        self.field(field_path).valid
    }

    pub fn field_message(&self, field_path: &str) -> &str {
        &self.field(field_path).message
    }

    pub fn validate(&mut self) -> bool {
        for field in &mut self.fields {
            field.valid = true;
            field.dirty = true;
        }

        self.update_validation();

        self.valid()
    }

    pub(crate) fn update_validation(&mut self) {
        match self.model.validate() {
            Ok(()) => self.clear_errors(),
            Err(errors) => self.add_errors("", None, &errors),
        }
    }

    pub(crate) fn update_validation_field(&mut self, field: &str) {
        match self.model.validate() {
            Ok(()) => self.clear_errors(),
            Err(errors) => self.add_errors("", Some(field), &errors),
        }
    }

    fn clear_errors(&mut self) {
        for field in &mut self.fields {
            field.message = String::new();
        }
    }

    fn add_errors(
        &mut self,
        prefix: &str,
        field_name_filter: Option<&str>,
        errors: &ValidationErrors,
    ) {
        fn generate_field_name(prefix: &str, field_name: &str) -> String {
            if prefix == "" {
                field_name.into()
            } else {
                format!("{}.{}", prefix, field_name)
            }
        }

        fn skip_field(field_name: &str, field_name_filter: Option<&str>) -> bool {
            if let Some(field_name_filter) = field_name_filter {
                field_name != field_name_filter
            } else {
                false
            }
        }

        for (field_name, error) in errors.errors() {
            if skip_field(field_name, field_name_filter) {
                continue;
            }

            let current_field_name = generate_field_name(prefix, field_name);

            match error {
                ValidationErrorsKind::Struct(errors) => {
                    self.add_errors(&current_field_name, field_name_filter, errors)
                }
                ValidationErrorsKind::List(_) => unimplemented!(),
                ValidationErrorsKind::Field(errors) => {
                    let field = self.field_mut(&current_field_name);

                    field.valid = false;

                    field.message = if let Some(message) = errors.first() {
                        message.to_string()
                    } else {
                        "Error".to_string()
                    }
                }
            }
        }
    }
}
