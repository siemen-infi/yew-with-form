use yew::prelude::*;

use crate::{Form, FormModel};

// TODO: figure this shit out...
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
