use std::cell::{Ref, RefMut};
use std::rc::Rc;
use std::{borrow::Borrow, cell::RefCell};

use yew::html::ImplicitClone;

use super::{form_model::FormModel, form_state::FormState};

#[derive(Clone)]
pub struct Form<T: FormModel> {
    state: Rc<RefCell<FormState<T>>>,
}

impl<T: FormModel> ImplicitClone for Form<T> {}

impl<T: FormModel> Form<T> {
    pub fn new(model: T) -> Self {
        Self {
            state: Rc::new(RefCell::new(FormState::new(model))),
        }
    }

    pub(crate) fn state(&self) -> Ref<FormState<T>> {
        self.state.as_ref().borrow()
    }

    pub(crate) fn state_mut(&self) -> RefMut<FormState<T>> {
        self.state.borrow_mut()
    }

    pub fn validate(&mut self) -> bool {
        self.state_mut().validate()
    }

    pub fn valid(&self) -> bool {
        self.state().valid()
    }

    pub fn field_value(&self, form_field_id: &str) -> String {
        self.state().field(form_field_id).value.to_owned()
    }

    pub fn field_valid(&self, form_field_id: &str) -> bool {
        self.state().field_valid(form_field_id)
    }

    pub fn field_message(&self, form_field_id: &str) -> String {
        self.state().field(form_field_id).message.to_owned()
    }

    pub fn set_field_value(&mut self, form_field_id: &str, value: &str) {
        self.state_mut().set_field_value(form_field_id, value);
    }

    pub fn model(&self) -> T {
        self.state().model().clone()
    }
}

impl<T: FormModel> PartialEq for Form<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.state, &other.state)
            || self.state().borrow().model == other.state().borrow().model
    }

    fn ne(&self, other: &Self) -> bool {
        self.state().model != other.state().model
    }
}
