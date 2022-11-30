use yew::{prelude::*, virtual_dom::AttrValue};

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
}

fn get_input_placeholder(props: &TextInputProps) -> String {
    if let Some(value) = &props.placeholder {
        value.clone()
    } else {
        "".to_string()
    }
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    let class = "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block flex-1 p-2.5";

    html! {
        <input
            id={props.id.clone()}
            {class}
            type={"text"}
            placeholder={get_input_placeholder(&props)}
        />
    }
}
