use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <h1 class="text-3xl font-bold">
            "Borang"
        </h1>
    }
}
