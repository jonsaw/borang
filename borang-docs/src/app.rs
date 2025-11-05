use borang::{
    Field, FieldErrorFor, Input, UseFormValidation, WithMessage,
    rules::{Email, Length, Required, Rules},
};
use leptos::prelude::*;
use leptos_i18n_router::I18nRoute;
use leptos_meta::*;
use leptos_router::{components::*, path};

use crate::i18n::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <I18nContextProvider>
            <LanguageMenu />
            <Router>
                <Routes fallback=|| "This page could not be found.">
                    <I18nRoute<Locale, _, _> view=|| view! { <Outlet /> }>
                        <Route path=path!("/") view=Example />
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

#[component]
pub fn Example() -> impl IntoView {
    let i18n = use_i18n();

    let form = UseFormValidation::new();

    let name = form.field("name", String::new()).with_validator(
        Rules::new()
            .add(WithMessage::new(Required, move |_| {
                t_string!(i18n, item_is_required, item = t_string!(i18n, name)).to_string()
            }))
            .add(WithMessage::new(Length::min(2).max(100), move |_| {
                t_string!(
                    i18n,
                    item_must_be_between,
                    item = t_string!(i18n, name),
                    min = 2,
                    max = 100
                )
                .to_string()
            })),
    );

    let email = form.field("email", String::new()).with_validator(
        Rules::new()
            .add(WithMessage::new(Required, move |_| {
                t_string!(i18n, item_is_required, item = t_string!(i18n, email)).to_string()
            }))
            .add(WithMessage::new(Email, move |_| {
                t_string!(i18n, item_is_not_valid, item = t_string!(i18n, email)).to_string()
            })),
    );

    {
        let form = form.clone();
        Effect::new(move |_| {
            i18n.get_locale();

            form.validate_all_fields();
        });
    }

    view! {
        <h1 class="text-3xl font-bold">{t!(i18n, form)}</h1>
        <form on:submit={
            let form = form.clone();
            move |event| {
                event.prevent_default();
                form.touch_all_fields();
                form.validate_all_fields();
            }
        }>
            <Field field=name.clone()>
                <Input attr:id="name" attr:placeholder=move || t_string!(i18n, name) />
                <FieldErrorFor field=name.clone() />
            </Field>
            <Field field=email.clone()>
                <Input
                    attr:id="email"
                    attr:r#type="email"
                    attr:placeholder=move || t_string!(i18n, email)
                />
                <FieldErrorFor field=email.clone() />
            </Field>
            <button type="submit">{t!(i18n, submit)}</button>
        </form>
    }
}
