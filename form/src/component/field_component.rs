use yew::{
    classes, html, virtual_dom::AttrValue, Callback, Classes, Component, Context, InputEvent,
    Properties,
};

use crate::FieldMessage;
use crate::Form;

use crate::FormModel;

fn default_input_type() -> AttrValue {
    AttrValue::from("text")
}

#[derive(Properties, PartialEq, Clone)]
pub struct FieldProperties<T: FormModel> {
    #[prop_or_else(default_input_type)]
    pub input_type: AttrValue,
    #[prop_or_else(|| AttrValue::from("text") )]
    pub form_field_id: AttrValue,
    pub form: Form<T>,
    #[prop_or_else(|| AttrValue::from(String::new()) )]
    pub placeholder: AttrValue,
    #[prop_or_else(|| classes!("form-control") )]
    pub class: Classes,
    #[prop_or_else(||  classes!("is-invalid") )]
    pub class_invalid: Classes,
    #[prop_or_else(||  classes!("is-valid") )]
    pub class_valid: Classes,
}

pub struct Field<T: FormModel> {
    pub input_type: AttrValue,
    pub form_field_id: AttrValue,
    pub form: Form<T>,
    pub placeholder: AttrValue,
    pub class: Classes,
    pub class_invalid: Classes,
    pub class_valid: Classes,
}

impl<T: FormModel> Component for Field<T> {
    type Message = FieldMessage;
    type Properties = FieldProperties<T>;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();

        Self {
            input_type: props.input_type.clone(),
            form_field_id: props.form_field_id.clone(),
            form: props.form.clone(),
            placeholder: props.placeholder.clone(),
            class: props.class.clone(),
            class_invalid: props.class_invalid.clone(),
            class_valid: props.class_valid.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut state = self.form.state_mut();

        state.update_validation();

        msg.apply_to_state(state);

        true
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        html! {
            <input
                class={self.class.clone()}
                id={self.form_field_id.clone()}
                type={self.input_type.clone()}
                placeholder={self.placeholder.clone()}
                value={self.form.field_value(&self.form_field_id)}
                onchange={ctx.link().callback(|e| FieldMessage::OnChange(e))}
            />
        }
    }
}
