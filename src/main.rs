use yew::prelude::*;

mod components;
use crate::components::organisms::person_form::PersonForm;

#[function_component(FormExample)]
fn form_example() -> Html {
    html! {
        <PersonForm />
    }
}

fn main() {
    yew::Renderer::<FormExample>::new().render();
}
