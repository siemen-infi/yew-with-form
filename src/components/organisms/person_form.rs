use crate::components::molecules::text_field::TextField;
use form::Form;
use form_derive::FormModel;
use gloo::console::log;
use validator::Validate;
use yew::prelude::*;

#[derive(FormModel, Validate, PartialEq, Clone)]
struct Person {
    #[validate(length(min = 1))]
    first_name: String,
    #[validate(length(min = 1))]
    last_name: String,
}

pub struct PersonForm {
    form: Form<Person>,
}

pub enum PersonFormMessage {
    Update,
    Submit,
}

impl Component for PersonForm {
    type Message = PersonFormMessage;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let model = Person {
            first_name: "Hello".to_owned(),
            last_name: "world".to_owned(),
        };

        Self {
            form: Form::new(model),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PersonFormMessage::Update => true, // Force component render update
            PersonFormMessage::Submit => {
                if self.form.validate() {
                    let current_model = self.form.model();

                    log!("submit!!!");
                    log!(current_model.first_name);
                    // gloo::console::log!(self.form.model())
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cb = ctx
            .link()
            .callback(|_: InputEvent| PersonFormMessage::Update);

        html! {
            <form>
                <TextField<Person>
                 form={&self.form}
                 form_field_id="first_name"
                 on_input={cb.clone()}
                 label_value={"Voornaam"}/>
                <TextField<Person>
                 form={&self.form}
                 form_field_id="last_name"
                 on_input={cb.clone()}
                 label_value={"Achternaam"}/>
                 <button
                 type="button"
                 onclick={ ctx.link().callback(|e: MouseEvent| {e.prevent_default(); PersonFormMessage::Submit}) }
             >
                 {"Submit"}
             </button>
            </form>
        }
    }
}
