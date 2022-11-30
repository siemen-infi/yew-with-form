use form::Form;
use form_derive::FormModel;
use validator::Validate;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

mod components;
use crate::components::molecules::text_field::TextField;
use crate::components::organisms::person_form::PersonForm;

use components::atoms::text_input::*;

enum Msg {
    AddOne,
    RemoveOne,
}

struct Model {
    value: u32,
}

#[derive(FormModel, Validate, PartialEq, Clone)]
struct Person {
    #[validate(length(min = 1, message = "a required"))]
    first_name: String,
    #[validate(length(min = 1, message = "b required"))]
    last_name: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
            Msg::RemoveOne => {
                self.value -= 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let text_input_props = TextInputProps {
            placeholder: Some("Very nice placeholder".to_string()),
            id: Some(AttrValue::from("id")),
        };

        let subtract_disabled = if self.value < 1 {
            Some(AttrValue::from("disabled"))
        } else {
            None
        };

        html! {
            <div>
                <PersonForm />
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Model>::new().render();
}
