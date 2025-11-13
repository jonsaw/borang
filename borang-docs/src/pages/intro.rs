use borang::{
    Field, FieldState, Form, FormComponent, FormComponentState, FromFieldValue, GetField,
    Validation, ValidationError,
};
use leptos::prelude::*;

use leptos_meta::Title;

use crate::components::code::Code;
use crate::i18n::*;
use crate::utils::validation_i18n::translate_validation_error;

#[component]
pub fn IntroPage() -> impl IntoView {
    let i18n = use_i18n();
    let example_code = r#"
    use borang::{Field, Form, FormComponent, FormValidation, FromFieldValue, ValidationError};
    use leptos::prelude::*;

    #[derive(Clone, Debug)]
    enum Country {
        Malaysia,
        Australia,
        England,
        Other,
    }

    impl Default for Country {
        fn default() -> Self {
            Country::Other
        }
    }

    impl FromFieldValue for Country {
        fn from_field_value(field_name: &str, value: &str) -> Result<Self, ValidationError> {
            match value {
                "Malaysia" => Ok(Country::Malaysia),
                "Australia" => Ok(Country::Australia),
                "England" => Ok(Country::England),
                _ => Err(ValidationError::new(
                    field_name,
                    "Invalid country selection",
                )),
            }
        }

        fn to_field_value(&self) -> String {
            match self {
                Country::Malaysia => "Malaysia",
                Country::Australia => "Australia",
                Country::England => "England",
                Country::Other => "Other",
            }
            .to_string()
        }
    }

    #[derive(FormValidation, Default, Clone)]
    struct ContactForm {
        #[validator(required)]
        name: String,

        #[validator(required, email)]
        email: String,

        #[validator(range(min = 18, max = 120))]
        age: u32,

        #[validator(required)]
        country: Country,
    }

    #[component]
    fn MyForm() -> impl IntoView {
        let contact = ContactForm {
            name: String::new(),
            email: String::new(),
            age: 0,
            country: Country::Other,
        };

        let form = Form::from(contact);

        let on_submit = {
            move |event: leptos::web_sys::SubmitEvent| {
                event.prevent_default();
                if form.validate() {
                    let data = form.data();
                    leptos::logging::log!("Name: {}", data.name);
                    leptos::logging::log!("Email: {}", data.email);
                    leptos::logging::log!("Age: {}", data.age);
                    leptos::logging::log!("Country: {:?}", data.country);
                }
            }
        };

        view! {
            <form on:submit=on_submit>
                <FormComponent form=form let:form_state>
                    <Field form=form name="name" let(value, set_value, state)>
                        <label for="name">"Name"</label>
                        <input
                            id="name"
                            type="text"
                            placeholder="Jed Saw"
                            bind:value=(value, set_value)
                            on:blur={
                                let mark_touched = state.mark_touched.clone();
                                move |_| mark_touched()
                            }
                        />
                        <Show when={
                            let state = state.clone();
                            move || state.has_error()
                        }>
                            <span class="error">
                                {
                                    let state = state.clone();
                                    move || { state.get_error().map(|e| e.message().to_string()) }
                                }
                            </span>
                        </Show>
                    </Field>

                    <Field form=form name="email" let(value, set_value, state)>
                        <label for="email">"Email"</label>
                        <input
                            id="email"
                            type="email"
                            placeholder="jed@borang.com"
                            bind:value=(value, set_value)
                            on:blur={
                                let mark_touched = state.mark_touched.clone();
                                move |_| mark_touched()
                            }
                        />
                        <Show when={
                            let state = state.clone();
                            move || state.has_error()
                        }>
                            <span class="error">
                                {
                                    let state = state.clone();
                                    move || { state.get_error().map(|e| e.message().to_string()) }
                                }
                            </span>
                        </Show>
                    </Field>

                    <Field form=form name="age" let(value, set_value, state)>
                        <label for="age">"Age"</label>
                        <input
                            id="age"
                            type="number"
                            placeholder="18"
                            bind:value=(value, set_value)
                            on:blur={
                                let mark_touched = state.mark_touched.clone();
                                move |_| mark_touched()
                            }
                        />
                        <Show when={
                            let state = state.clone();
                            move || state.has_error()
                        }>
                            <span class="error">
                                {
                                    let state = state.clone();
                                    move || { state.get_error().map(|e| e.message().to_string()) }
                                }
                            </span>
                        </Show>
                    </Field>

                    <Field form=form name="country" let(value, set_value, state)>
                        <label for="country">"Country"</label>
                        <select
                            id="country"
                            bind:value=(value, set_value)
                            on:blur={
                                let mark_touched = state.mark_touched.clone();
                                move |_| mark_touched()
                            }
                        >
                            <option value="Malaysia">"Malaysia"</option>
                            <option value="Australia">"Australia"</option>
                            <option value="England">"England"</option>
                            <option value="Other">"Other"</option>
                        </select>
                        <Show when={
                            let state = state.clone();
                            move || state.has_error()
                        }>
                            <span class="error">
                                {
                                    let state = state.clone();
                                    move || { state.get_error().map(|e| e.message().to_string()) }
                                }
                            </span>
                        </Show>
                    </Field>

                    <div>
                        <p>"Form valid: " {move || form_state.valid.get().to_string()}</p>
                        <p>"Form dirty: " {move || form_state.dirty.get().to_string()}</p>
                        <p>"Form touched: " {move || form_state.touched.get().to_string()}</p>
                    </div>
                    <button type="submit">"Submit"</button>
                </FormComponent>
            </form>
        }
    }"#;

    view! {
        <Title text="Introduction" />
        <article class="flex flex-col pt-16 pb-10 m-4 h-full md:m-8">
            <h1>{t!(i18n, intro.title)}</h1>
            <p class="mb-4">{t!(i18n, intro.description)}</p>
            <p class="mb-2">{t!(i18n, example_here)}</p>
            <div class="p-6 mb-4 rounded-lg border border-border bg-muted text-muted-foreground">
                <ExampleForm />
            </div>
            <Code
                code=example_code
                language="rust"
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
            />
        </article>
    }
}

#[derive(Default, Clone, Debug)]
enum Country {
    Malaysia,
    Australia,
    England,
    #[default]
    Other,
}

impl FromFieldValue for Country {
    fn from_field_value(field_name: &str, value: &str) -> Result<Self, borang::ValidationError> {
        let i18n = use_i18n();
        match value {
            "Malaysia" => Ok(Country::Malaysia),
            "Australia" => Ok(Country::Australia),
            "England" => Ok(Country::England),
            _ => Err(ValidationError::new(
                field_name,
                t_string!(i18n, invalid_country),
            )),
        }
    }

    fn to_field_value(&self) -> String {
        match self {
            Country::Malaysia => "Malaysia",
            Country::Australia => "Australia",
            Country::England => "England",
            Country::Other => "Other",
        }
        .to_string()
    }
}

#[derive(Validation, Default, Clone)]
struct ContactForm {
    #[validator(required)]
    name: String,

    #[validator(required, email)]
    email: String,

    #[validator(range(min = 18, max = 120))]
    age: u32,

    #[validator(required)]
    country: Country,
}

#[component]
pub fn FieldError(
    state: FieldState,
    #[prop(into)] field_name: Signal<&'static str>,
) -> impl IntoView {
    let i18n = use_i18n();

    let error_message = {
        let state = state.clone();
        move || {
            state
                .get_error()
                .map(|e| translate_validation_error(i18n, &e, field_name.get()))
        }
    };

    view! {
        <Show when={
            let state = state.clone();
            move || state.has_error()
        }>
            <span class="block mt-1 text-sm text-red-500">{error_message()}</span>
        </Show>
    }
}

#[component]
fn ExampleForm() -> impl IntoView {
    let i18n = use_i18n();
    let contact = ContactForm {
        name: String::new(),
        email: String::new(),
        age: 0,
        country: Country::Other,
    };

    let form = Form::from(contact);

    let on_submit = {
        move |event: leptos::web_sys::SubmitEvent| {
            event.prevent_default();
            if form.validate() {
                let data = form.data();
                leptos::logging::log!("Name: {}", data.name);
                leptos::logging::log!("Email: {}", data.email);
                leptos::logging::log!("Age: {}", data.age);
                leptos::logging::log!("Country: {:?}", data.country);
            }
        }
    };

    view! {
        <form on:submit=on_submit>
            <FormComponent form=form let(form_state: FormComponentState)>
                <Field form=form name="name" let(value, set_value, state)>
                    <div class="mb-4">
                        <label for="name" class="block mb-2 text-sm font-medium">
                            {t!(i18n, name)}
                        </label>
                        <input
                            id="name"
                            type="text"
                            placeholder="Jed Saw"
                            class="py-2 px-3 w-full rounded-md border focus:ring-2 focus:outline-none border-border bg-background focus:ring-primary"
                            class:border-red-500={
                                let state = state.clone();
                                move || state.has_error()
                            }
                            bind:value=(value, set_value)
                            on:blur={
                                let state = state.clone();
                                move |_| (state.mark_touched)()
                            }
                        />
                        <FieldError
                            state=state
                            field_name=Signal::derive(move || t_string!(i18n, name))
                        />
                    </div>
                </Field>

                <Field form=form name="email" let(value, set_value, state)>
                    <div class="mb-4">
                        <label for="email" class="block mb-2 text-sm font-medium">
                            {t!(i18n, email)}
                        </label>
                        <input
                            id="email"
                            type="email"
                            placeholder="jed@borang.com"
                            class="py-2 px-3 w-full rounded-md border focus:ring-2 focus:outline-none border-border bg-background focus:ring-primary"
                            class:border-red-500={
                                let state = state.clone();
                                move || state.has_error()
                            }
                            bind:value=(value, set_value)
                            on:blur={
                                let state = state.clone();
                                move |_| (state.mark_touched)()
                            }
                        />
                        <FieldError
                            state=state
                            field_name=Signal::derive(move || t_string!(i18n, email))
                        />
                    </div>
                </Field>

                <Field form=form name="age" let(value, set_value, state)>
                    <div class="mb-4">
                        <label for="age" class="block mb-2 text-sm font-medium">
                            {t!(i18n, age)}
                        </label>
                        <input
                            id="age"
                            type="number"
                            placeholder="18"
                            class="py-2 px-3 w-full rounded-md border focus:ring-2 focus:outline-none border-border bg-background focus:ring-primary"
                            class:border-red-500={
                                let state = state.clone();
                                move || state.has_error()
                            }
                            bind:value=(value, set_value)
                            on:blur={
                                let state = state.clone();
                                move |_| (state.mark_touched)()
                            }
                        />
                        <FieldError
                            state=state
                            field_name=Signal::derive(move || t_string!(i18n, age))
                        />
                    </div>
                </Field>

                <Field form=form name="country" let(value, set_value, state)>
                    <div class="mb-4">
                        <label for="country" class="block mb-2 text-sm font-medium">
                            {t!(i18n, country)}
                        </label>
                        <select
                            id="country"
                            class="py-2 px-3 w-full rounded-md border focus:ring-2 focus:outline-none border-border bg-background focus:ring-primary"
                            class:border-red-500={
                                let state = state.clone();
                                move || state.has_error()
                            }
                            bind:value=(value, set_value)
                            on:blur={
                                let state = state.clone();
                                move |_| (state.mark_touched)()
                            }
                        >
                            <option value="Malaysia">"Malaysia"</option>
                            <option value="Australia">"Australia"</option>
                            <option value="England">"England"</option>
                            <option value="Other">"Other"</option>
                        </select>
                        <FieldError
                            state=state
                            field_name=Signal::derive(move || t_string!(i18n, country))
                        />
                    </div>
                </Field>

                <div class="p-4 mt-6 mb-4 space-y-0.5 rounded-md border bg-muted/50 border-border">
                    <p class="font-mono text-sm">
                        <span class="font-semibold">"Form valid: "</span>
                        <span class=move || {
                            if form_state.valid.get() { "text-green-600" } else { "text-red-600" }
                        }>{move || form_state.valid.get().to_string()}</span>
                    </p>
                    <p class="font-mono text-sm">
                        <span class="font-semibold">"Form dirty: "</span>
                        <span class=move || {
                            if form_state.dirty.get() {
                                "text-blue-600"
                            } else {
                                "text-muted-foreground"
                            }
                        }>{move || form_state.dirty.get().to_string()}</span>
                    </p>
                    <p class="font-mono text-sm">
                        <span class="font-semibold">"Form touched: "</span>
                        <span class=move || {
                            if form_state.touched.get() {
                                "text-blue-600"
                            } else {
                                "text-muted-foreground"
                            }
                        }>{move || form_state.touched.get().to_string()}</span>
                    </p>
                </div>

                <div class="p-4 mt-6 mb-4 space-y-0.5 rounded-md border bg-muted/50 border-border">
                    <GetField form=form name="name" let(value)>
                        <p class="font-mono text-sm">
                            <span class="font-semibold">"Name: "</span>
                            <span>{value}</span>
                        </p>
                    </GetField>
                    <GetField form=form name="email" let(value)>
                        <p class="font-mono text-sm">
                            <span class="font-semibold">"Email: "</span>
                            <span>{value}</span>
                        </p>
                    </GetField>
                    <GetField form=form name="age" let(value)>
                        <p class="font-mono text-sm">
                            <span class="font-semibold">"Age: "</span>
                            <span>{value}</span>
                        </p>
                    </GetField>
                    <GetField form=form name="country" let(value)>
                        <p class="font-mono text-sm">
                            <span class="font-semibold">"Country: "</span>
                            <span>{value}</span>
                        </p>
                    </GetField>
                </div>

                <button
                    type="submit"
                    class="py-2 px-4 w-full font-medium rounded-md border transition-colors bg-primary text-primary-foreground border-border hover:bg-primary/90"
                >
                    {t!(i18n, submit)}
                </button>
            </FormComponent>
        </form>
    }
}
