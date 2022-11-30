use yew::{prelude::*, virtual_dom::AttrValue};

#[derive(Properties, PartialEq)]
pub struct LabelProps {
    pub for_attr: AttrValue,
    pub children: Children,
}

#[function_component(Label)]
pub fn label(props: &LabelProps) -> Html {
    let class = "text-md block text-gray-900 align-middle flex-1";

    html! {
        <label
            {class}
            for={props.for_attr.clone()}>
            {props.children.clone()}
        </label>
    }
}
