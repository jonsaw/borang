use leptos::{portal::Portal, prelude::*};

use crate::{
    components::{icons, menu},
    i18n::*,
};

#[component]
pub fn LanguageMenu(
    #[prop(default = menu::Positioning::BottomEnd)] positioning: menu::Positioning,
    children: Children,
) -> impl IntoView {
    let i18n = use_i18n();

    let languages = || vec![("English", Locale::en), ("Bahasa Malaysia", Locale::ms)];

    view! {
        <menu::Menu positioning=positioning>
            <menu::Trigger>{children()}</menu::Trigger>
            <Portal>
                <menu::Content>
                    {languages()
                        .into_iter()
                        .map(|(name, code)| {
                            view! {
                                <menu::Item>
                                    <button
                                        on:click=move |_| {
                                            i18n.set_locale(code);
                                        }
                                        class="flex justify-between py-1.5 px-2 w-full cursor-pointer align-center"
                                    >
                                        {name}
                                        <Show when=move || { code == i18n.get_locale() }>
                                            <icons::Check class="w-4"></icons::Check>
                                        </Show>
                                    </button>
                                </menu::Item>
                            }
                        })
                        .collect_view()}
                </menu::Content>
            </Portal>
        </menu::Menu>
    }
}
