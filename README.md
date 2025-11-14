# Borang

Borang is a Leptos library for building web forms with validation.

## Getting Started

Install Borang using Cargo:

```sh
cargo add borang
```

See [demo](https://borang-leptos.vercel.app).

## Basic Usage

Simple example:

```rust
use borang::{Form, FormComponent, Field, Input, Validation};
use leptos::prelude::*;

#[derive(Validation, Default, Clone)]
struct ContactForm {
    #[validator(required)]
    name: String,

    #[validator(required, email)]
    email: String,
}

#[component]
fn App() -> impl IntoView {
    let contact = ContactForm {
        name: String::new(),
        email: String::new(),
    };

    let form = Form::from(contact);

    let on_submit = {
        move |event: leptos::web_sys::SubmitEvent| {
            event.prevent_default();
            if form.validate() {
                let data = form.data();
                leptos::logging::log!("Name: {}", data.name);
                leptos::logging::log!("Email: {}", data.email);
            }
        }
    };

    view! {
        <form on:submit=on_submit>
            <FormComponent form=form>
                <Field form=form name="name" let(_value, field_state)>
                    <label for="name">"Name"</label>
                    <Input
                        state=field_state
                        attr:id="name"
                        attr:type="text"
                        attr:placeholder="Jed Saw"
                    />
                </Field>

                <Field form=form name="email" let(_value, field_state)>
                    <label for="email">"Email"</label>
                    <Input
                        state=field_state
                        attr:id="email"
                        attr:type="email"
                        attr:placeholder="jed@borang.com"
                    />
                </Field>

                <button type="submit">"Submit"</button>
            </FormComponent>
        </form>
    }
}

fn main() {
    mount_to_body(|| {
        view! { <App /> }
    })
}
```
