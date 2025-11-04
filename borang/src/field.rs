use leptos::prelude::*;

use crate::ValidatedField;

#[component]
pub fn Field<T>(field: ValidatedField<T>, children: Children) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
{
    provide_context(field);

    view! {
        {children()}
    }
}

#[component]
pub fn FieldError() -> impl IntoView {
    let field = expect_context::<ValidatedField<String>>();
    let error_message = field.error_message();

    view! {
        <div class="error">
            {error_message}
        </div>
    }
}

#[component]
pub fn FieldErrorFor<T>(#[prop(into)] field: ValidatedField<T>) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
{
    let error_message = field.error_message();

    view! {
        <div class="error">
            {error_message}
        </div>
    }
}
