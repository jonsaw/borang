use leptos::prelude::*;

use crate::{validation::FormValidation, FieldState};

/// Select component for form fields.
///
/// This component binds the select value and sets up mark touched
/// on blur to the FieldState.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Field form=form name="country" let:field_state>
///         <Select state=field_state class="select-class">
///             <option value="us">"United States"</option>
///             <option value="uk">"United Kingdom"</option>
///             <option value="ca">"Canada"</option>
///         </Select>
///     </Field>
/// }
/// ```
#[component]
pub fn Select<T>(
    state: FieldState<T>,
    #[prop(into, optional)] class: &'static str,
    children: Children,
) -> impl IntoView
where
    T: FormValidation + Default + Clone + Send + Sync + 'static,
{
    let value = state.value();
    view! {
        <select bind:value=value class=class on:blur=move |_| state.mark_touched()>
            {children()}
        </select>
    }
}
