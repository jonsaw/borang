use leptos::prelude::*;

use crate::ValidatedField;

#[component]
pub fn Input() -> impl IntoView {
    let field = expect_context::<ValidatedField<String>>();

    view! {
        <input
            value=move || field.value.get()
            on:input={
                let field = field.clone();
                move |ev| {
                    let value = event_target_value(&ev);
                    field.value.set(value);
                    field.mark_dirty();
                    field.validate();
                }
            }
            on:blur={
                let field = field.clone();
                move |_| {
                    field.mark_touched();
                    field.validate();
                }
            }
        />
    }
}
