use std::cell::RefMut;

use wasm_bindgen::JsCast;
use yew::Event;

use crate::{form_state::FormState, FormModel};

pub enum FieldMessage {
    OnChange(Event),
}

impl FieldMessage {
    pub(super) fn apply_to_state<T: FormModel>(&self, mut state: RefMut<FormState<T>>) {
        match self {
            FieldMessage::OnChange(event) => {
                let target = event
                    .target()
                    .expect("InputEvent with no target")
                    .dyn_into::<web_sys::HtmlInputElement>()
                    .unwrap();

                let value = target.value();

                state.set_field_value(target.id().as_str(), &value);
            }
        }
    }
}
