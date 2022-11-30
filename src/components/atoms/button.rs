use yew::{prelude::*, virtual_dom::AttrValue};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub disabled: Option<AttrValue>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let class;
    let disabled;
    let onclick = props.onclick.clone();

    match props.disabled {
        Some(_) => {
            class = "bg-blue-500 text-white font-bold py-2 px-4 rounded opacity-50 border border-blue-700 cursor-not-allowed";
            disabled = true;
        }
        None => {
            class = "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 border border-blue-700 rounded";
            disabled = false;
        }
    };

    html! {
        <button
            {class}
            {disabled}
            {onclick}>
            {props.label.clone()}
        </button>
    }
}
