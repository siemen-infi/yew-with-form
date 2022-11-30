use crate::components::atoms::{checkbox::*, label::*};

use yew::{prelude::*, virtual_dom::AttrValue};

#[derive(Properties, PartialEq)]
pub struct CheckboxFieldProps {
    pub id: AttrValue,
    pub label_value: String,
}

#[function_component(CheckboxField)]
pub fn checkbox_field(props: &CheckboxFieldProps) -> Html {
    html! {
        <div>
            <Label for_attr={props.id.clone()}>
                {&props.label_value}
            </Label>
            <Checkbox id={props.id.clone()}/>
        </div>
    }
}
