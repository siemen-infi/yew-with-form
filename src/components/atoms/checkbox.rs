use yew::{function_component, html, virtual_dom::AttrValue, Html, Properties};

#[derive(Properties, PartialEq, Eq)]
pub struct CheckboxProps {
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub disabled: Option<AttrValue>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
}

#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let class;
    let disabled;

    match props.disabled {
        Some(_) => {
            class = "w-4 h-4 text-blue-600 bg-gray-100 rounded border-gray-300 focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600";
            disabled = true;
        }
        _ => {
            class = "w-4 h-4 text-blue-600 bg-gray-100 rounded border-gray-300 focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600";
            disabled = false;
        }
    };

    html! {
        <input type="checkbox"
            id={props.id.clone()}
            {class}
            {disabled}/>
    }
}
