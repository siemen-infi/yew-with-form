use crate::components::atoms::{label::*, text_input::*};
use form::{component::Field, Form, FormModel};
use yew::prelude::*;

use super::checkbox_field::_CheckboxFieldProps::label_value;

#[derive(Properties, PartialEq)]
pub struct TextFieldProps<T: FormModel> {
    pub form_field_id: String,
    pub form: Form<T>,
    pub on_input: Option<Callback<InputEvent>>,
    #[prop_or(None)]
    pub label_value: Option<String>,
}

fn optional_label<T>(props: &TextFieldProps<T>) -> Html
where
    T: FormModel,
{
    if let Some(label) = &props.label_value {
        html! {
            <Label for_attr={props.form_field_id.clone()}>
                {{label}}
            </Label>
        }
    } else {
        html! {<></>}
    }
}

fn optional_validation_error<T>(props: &TextFieldProps<T>) -> Html
where
    T: FormModel,
{
    let field_message = props.form.field_message(&props.form_field_id);

    if !field_message.is_empty() {
        html! {
        <span class="text-red-500">
            {field_message}
        </span>
        }
    } else {
        html!(
            <></>
        )
    }
}

#[function_component(TextField)]
pub fn text_field<T>(props: &TextFieldProps<T>) -> Html
where
    T: FormModel,
{
    html! {
        <>
            {optional_label(props)}
            <Field<T>
                form={props.form.clone()}
                form_field_id={props.form_field_id.clone()}
                on_input={props.on_input.clone()}>
            </Field<T>>
            {optional_validation_error(props)}
        </>
    }
}
