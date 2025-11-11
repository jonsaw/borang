use biji_ui::{cn, components::menu};
use leptos::prelude::*;

pub use menu::Menu;
pub use menu::Positioning;

#[component]
pub fn Trigger(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <menu::Trigger class=cn!(
            class, "flex justify-center items-center w-6 h-6 rounded-md transition cursor-pointer"
        )>{children()}</menu::Trigger>
    }
}

#[component]
pub fn Content(
    children: ChildrenFn,
    /// Optional CSS class to apply to both show and hide classes
    #[prop(into, optional)]
    class: String,
    /// Optional CSS class to apply if `when == true`
    #[prop(into, optional)]
    show_class: String,
    /// Optional CSS class to apply if `when == false`
    #[prop(into, optional)]
    hide_class: String,
) -> impl IntoView {
    view! {
        <menu::Content
            class=cn!(
                class, "flex z-50 flex-col p-1 w-40 rounded-md border shadow-md transition focus:outline-none min-w-[8rem] border-border bg-background"
            )
            show_class=cn!(show_class, "opacity-100 duration-150 ease-in")
            hide_class=cn!(hide_class, "opacity-0 duration-200 ease-out")
        >
            {children()}
        </menu::Content>
    }
}

#[component]
pub fn Item(
    #[prop(default = false)] disabled: bool,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <menu::Item
            class=cn!(
                class, "flex items-center text-sm rounded-sm cursor-pointer outline-none select-none focus:outline-none hover:bg-active hover:text-active-foreground !ring-0 !ring-transparent data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[highlighted]:bg-muted"
            )
            disabled=disabled
        >
            {children()}
        </menu::Item>
    }
}
