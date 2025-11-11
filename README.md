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
use borang::{Form, FormComponent, Field};
use leptos::prelude::*;

#[derive(Clone)]
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
        let form = form.clone();
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
                <Field<ContactForm, _, _> name="name" let(value, set_value, state)>
                    <label for="name">"Name"</label>
                    <input
                        id="name"
                        type="text"
                        placeholder="Jed Saw"
                        bind:value=(value, set_value)
                        on:blur={
                            let state = state.clone();
                            move |_| (state.mark_touched)()
                        }
                    />
                </Field<ContactForm, _, _>>

                <Field<ContactForm, _, _> name="email" let(value, set_value, state)>
                    <label for="email">"Email"</label>
                    <input
                        id="email"
                        type="email"
                        placeholder="jed@borang.com"
                        bind:value=(value, set_value)
                        on:blur={
                            let state = state.clone();
                            move |_| (state.mark_touched)()
                        }
                    />
                </Field<ContactForm, _, _>>

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
