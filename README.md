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
use borang::{Form, FormComponent, Field, Validation};
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
                <Field form=form name="name" let(value, field_state)>
                    <label for="name">"Name"</label>
                    <input
                        id="name"
                        type="text"
                        placeholder="Jed Saw"
                        bind:value=value
                        on:blur=move |_| field_state.mark_touched()
                    />
                </Field>

                <Field form=form name="email" let(value, field_state)>
                    <label for="email">"Email"</label>
                    <input
                        id="email"
                        type="email"
                        placeholder="jed@borang.com"
                        bind:value=value
                        on:blur=move |_| field_state.mark_touched()
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
