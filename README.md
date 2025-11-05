# Borang

Borang is a Leptos library for building web forms with validation and error handling.

## Getting Started

Install Borang using Cargo:

```sh
cargo add borang
```

## Basic Usage

Simple example:

```rust
use borang::{
    Field, FieldErrorFor, Input, UseFormValidation,
    rules::{Email, Required, Rules},
};
use leptos::prelude::*;

#[component]
fn Form() -> impl IntoView {
    let form = UseFormValidation::new();

    let name = form.field("name", String::new()).with_validator(Required);
    let email = form
        .field("email", String::new())
        .with_validator(Rules::new().add(Required).add(Email));

    view! {
        <form on:submit={
            let form = form.clone();
            move |event| {
                event.prevent_default();
                form.touch_all_fields();
                form.validate_all_fields();
            }
        }>
            <Field field=name.clone()>
                <label for="name">"Name"</label>
                <Input attr:id="name" attr:placeholder="Jed Saw" />
                <FieldErrorFor field=name.clone() />

            </Field>
            <Field field=email.clone()>
                <label for="email">"Email"</label>
                <Input attr:id="email" attr:placeholder="jed@borang.com" />
                <FieldErrorFor field=email.clone() />
            </Field>
            <button type="submit">Submit</button>
        </form>
    }
}

fn main() {
    mount_to_body(|| {
        view! { <Form /> }
    })
}
```
