use leptos::{portal::Portal, prelude::*};
use leptos_use::{use_color_mode_with_options, ColorMode, UseColorModeOptions, UseColorModeReturn};

use crate::{
    components::{icons, menu},
    i18n::*,
};

#[component]
pub fn ActiveThemeIcon() -> impl IntoView {
    view! {
        <icons::Sun class="w-5 h-5 dark:hidden"></icons::Sun>
        <icons::Moon class="hidden w-5 h-5 dark:block"></icons::Moon>
    }
}

#[component]
pub fn ThemeMode(
    #[prop(default = menu::Positioning::BottomEnd)] positioning: menu::Positioning,
    children: Children,
) -> impl IntoView {
    let UseColorModeReturn { mode, set_mode, .. } =
        use_color_mode_with_options(UseColorModeOptions::default().emit_auto(true));

    let modes = || {
        let i18n = use_i18n();
        let light_text = t_string!(i18n, theme_light);
        let dark_text = t_string!(i18n, theme_dark);
        let system_text = t_string!(i18n, theme_system);
        [
            (light_text, &ColorMode::Light),
            (dark_text, &ColorMode::Dark),
            (system_text, &ColorMode::Auto),
        ]
    };

    view! {
        <menu::Menu positioning=positioning>
            <menu::Trigger>{children()}</menu::Trigger>
            <Portal>
                <menu::Content>
                    {modes()
                        .into_iter()
                        .map(|(title, m)| {
                            view! {
                                <menu::Item>
                                    <button
                                        on:click=move |_| { set_mode.set(m.clone()) }
                                        class="flex justify-between py-1.5 px-2 w-full cursor-pointer align-center"
                                    >
                                        <div class="flex gap-2">
                                            {match m.clone() {
                                                ColorMode::Light => {
                                                    view! { <icons::Sun class="w-4"></icons::Sun> }.into_any()
                                                }
                                                ColorMode::Dark => {
                                                    view! { <icons::Moon class="w-4"></icons::Moon> }.into_any()
                                                }
                                                _ => {
                                                    view! { <icons::SunMoon class="w-4"></icons::SunMoon> }
                                                        .into_any()
                                                }
                                            }} {title}
                                        </div>
                                        <Show when=move || m.clone() == mode.get()>
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
