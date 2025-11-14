use leptos::prelude::*;

use crate::{validation::FormValidation, FieldState};

/// Input component for form fields.
///
/// This component binds the input value and sets up mark touched
/// on blur to the FieldState.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Field form=form name="email" let:field_state>
///         <Input state=field_state class="input-class" />
///     </Field>
/// }
/// ```
#[component]
pub fn Input<T>(state: FieldState<T>, #[prop(into, optional)] class: &'static str) -> impl IntoView
where
    T: FormValidation + Default + Clone + Send + Sync + 'static,
{
    let value = state.value();
    view! { <input bind:value=value class=class on:blur=move |_| state.mark_touched() /> }
}
