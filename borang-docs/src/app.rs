use leptos::prelude::*;
use leptos_i18n_router::I18nRoute;
use leptos_meta::*;
use leptos_router::{components::*, path};

use crate::{
    i18n::*,
    pages::{intro::IntroPage, scaffold::DocsScaffold},
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <I18nContextProvider>
            <Router>
                <Routes fallback=|| "This page could not be found.">
                    <I18nRoute<Locale, _, _> view=|| view! { <Outlet /> }>
                        <Route path=path!("/") view=|| view! { <Redirect path="/docs/intro" /> } />
                        <ParentRoute path=path!("/docs") view=DocsScaffold>
                            <Route path=path!("/intro") view=IntroPage />
                        </ParentRoute>
                    </I18nRoute<Locale, _, _>>
                </Routes>
            </Router>
        </I18nContextProvider>
    }
}

#[component]
pub fn LanguageMenu() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <nav>
            <button on:click=move |_| i18n.set_locale(Locale::en)>"English"</button>
            <button on:click=move |_| i18n.set_locale(Locale::ms)>"Bahasa Malaysia"</button>
        </nav>
    }
}
