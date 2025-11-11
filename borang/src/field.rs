use leptos::prelude::*;
use std::sync::Arc;

use super::form::Form;
use super::validation::{FormValidation, ValidationError};

/// State object provided by Field component containing error, dirty, touched signals and mark_touched callback
#[derive(Clone)]
pub struct FieldState {
    /// Current validation error for this field
    pub err: Signal<Option<ValidationError>>,
    /// True if field value differs from initial value
    pub dirty: Signal<bool>,
    /// True if field has been marked as touched
    pub touched: Signal<bool>,
    /// Callback to mark field as touched (typically called on blur)
    pub mark_touched: Arc<dyn Fn() + Send + Sync>,
}

impl FieldState {
    pub fn has_error(&self) -> bool {
        self.err.get().is_some()
    }

    pub fn get_error(&self) -> Option<ValidationError> {
        self.err.get()
    }
}

/// Field component that binds to a specific field in the parent form
///
/// This component:
/// - Registers the field with the parent form via context
/// - Creates reactive value and error signals from form state
/// - Provides a setter that updates form state and triggers validation
/// - Passes value and error to children via the children function
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Field<MyForm, _, _> name="email" let(value, set_value, state)>
///         <input
///             type="email"
///             bind:value=(value, set_value)
///             on:blur=move |_| (state.mark_touched)()
///         />
///         <Show when=move || state.err.get().is_some()>
///             <span class="error">{move || state.err.get().map(|e| e.message().to_string())}</span>
///         </Show>
///         <Show when=move || state.dirty.get()>
///             <span class="info">"Field has been modified"</span>
///         </Show>
///     </Field<MyForm, _, _>>
/// }
/// ```
#[component]
pub fn Field<T, F, IV>(
    /// The name of the field (must match a field in the form struct)
    name: &'static str,
    /// Children function that receives (value, set_value, state)
    children: F,
    /// Phantom data to use the type parameter T
    #[prop(default = std::marker::PhantomData)]
    _phantom: std::marker::PhantomData<T>,
) -> impl IntoView
where
    T: FormValidation + Default + Clone + Send + Sync + 'static,
    F: Fn(Signal<String>, WriteSignal<String>, FieldState) -> IV + 'static,
    IV: IntoView,
{
    // Get form from context
    let form = expect_context::<Form<T>>();
    let state = form.state_signal();

    // Register field with form state (get or create the field signal)
    let field_signal = state.update_untracked(|s| s.get_or_create_field(name));

    // Create reactive value signal for this field
    let value = Signal::derive({
        let field_signal = field_signal.clone();
        move || field_signal.value.get()
    });

    // Get the write signal directly from the field signal
    let set_value = field_signal.value.write_only();

    // Set up an effect to handle reactive validation when value changes
    // This enables immediate validation feedback as users type
    {
        let form = form.clone();
        let name = name.to_string();
        let field_signal = field_signal.clone();

        Effect::new(move || {
            // Track the value - this effect runs whenever the value changes
            let _value = field_signal.value.get();

            // Trigger field-level validation
            // This updates the error state reactively, which causes the error signal
            // to update automatically, providing immediate feedback to the user
            form.validate_field(&name);
        });
    }

    // Create mark_touched callback
    let mark_touched = {
        let name = name.to_string();
        Arc::new(move || {
            state.update(|s| {
                s.touched.insert(name.clone(), true);
            });
        }) as Arc<dyn Fn() + Send + Sync>
    };

    // Create reactive error signal for this field
    let error = Signal::derive({
        let name = name.to_string();
        move || state.get().errors.get(&name).cloned()
    });

    // Create reactive dirty signal for this field
    let dirty = Signal::derive({
        let name = name.to_string();
        move || state.get().is_field_dirty(&name)
    });

    // Create reactive touched signal for this field
    let touched = Signal::derive({
        let name = name.to_string();
        move || state.get().is_field_touched(&name)
    });

    // Create FieldState object
    let state = FieldState {
        err: error,
        dirty,
        touched,
        mark_touched,
    };

    // Pass value, setter, and state to children
    // This enables the `let(value, set_value, state)` syntax
    children(value, set_value, state)
}

/// GetField component that only reads a field value from the parent form
///
/// This is a simplified version of Field that only provides read access to the field value.
/// Use this when you only need to display field values without any editing capabilities.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <GetField<MyForm, _> name="email" let(value)>
///         <p>"Email: " {move || value.get()}</p>
///     </GetField<MyForm, _>>
/// }
/// ```
#[component]
pub fn GetField<T, F, IV>(
    /// The name of the field (must match a field in the form struct)
    name: &'static str,
    /// Children function that receives (value)
    children: F,
    /// Phantom data to use the type parameter T
    #[prop(default = std::marker::PhantomData)]
    _phantom: std::marker::PhantomData<T>,
) -> impl IntoView
where
    T: FormValidation + Default + Clone + Send + Sync + 'static,
    F: Fn(Signal<String>) -> IV + 'static,
    IV: IntoView,
{
    // Get form from context
    let form = expect_context::<Form<T>>();
    let state = form.state_signal();

    // Register field with form state (get or create the field signal)
    let field_signal = state.update_untracked(|s| s.get_or_create_field(name));

    // Create reactive value signal for this field
    let value = Signal::derive({
        let field_signal = field_signal.clone();
        move || field_signal.value.get()
    });

    // Pass value to children
    // This enables the `let(value)` syntax
    children(value)
}
