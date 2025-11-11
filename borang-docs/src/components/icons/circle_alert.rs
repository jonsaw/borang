use leptos::prelude::*;

#[component]
pub fn CircleAlert(#[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <svg
            viewBox="0 0 16 16"
            fill="currentColor"
            data-slot="icon"
            aria-hidden="true"
            class=class
        >
            <path
                d="M8 15A7 7 0 1 0 8 1a7 7 0 0 0 0 14ZM8 4a.75.75 0 0 1 .75.75v3a.75.75 0 0 1-1.5 0v-3A.75.75 0 0 1 8 4Zm0 8a1 1 0 1 0 0-2 1 1 0 0 0 0 2Z"
                clip-rule="evenodd"
                fill-rule="evenodd"
            />
        </svg>
    }
}
