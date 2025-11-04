use borang::{
    rules::{Required, Rules},
    Field, FieldErrorFor, Form, Input, UseFormValidation, WithMessage,
};
use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let form = UseFormValidation::new();

    let email = form
        .field("email", String::new())
        .with_validator(Rules::new().add(WithMessage::new(Required, "Email is required")));

    view! {
        <h1 class="text-3xl font-bold">
            "Borang"
        </h1>
        <Form>
            <Field field=email.clone()>
                <Input attr:id="email" attr:r#type="email" attr:placeholder="Email" />
                <FieldErrorFor field=email />
            </Field>
            <button type="submit">Submit</button>
        </Form>
    }
}
