use leptos::prelude::*;

#[component]
pub fn Form(children: Children) -> impl IntoView {
    view! {
        <form>
            {children()}
        </form>
    }
}
