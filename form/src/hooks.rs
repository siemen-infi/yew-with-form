use yew::{prelude::*, use_state, Callback, UseStateHandle};

use crate::{Form, FormModel};

pub fn use_form<T>(model: T)
where
    T: FormModel + Copy,
{
    use_effect_with_deps(
        move |model| {
            let form = Form::new(model.clone());

            move || drop(form)
        },
        model,
    );
}
