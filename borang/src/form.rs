use leptos::prelude::*;
use std::collections::HashMap;

use super::validation::{FieldSignal, FormValidation, ValidationError};

/// State object provided by FormComponent containing form values, errors, and status
#[derive(Clone)]
pub struct FormComponentState {
    /// Current values of all fields
    pub values: Signal<HashMap<String, String>>,
    /// Current validation errors for all fields
    pub errors: Signal<HashMap<String, ValidationError>>,
    /// True if any field is dirty (differs from initial value)
    pub dirty: Signal<bool>,
    /// True if any field has been touched
    pub touched: Signal<bool>,
    /// True if form has no validation errors
    pub valid: Signal<bool>,
}

/// Internal form state that stores individual field signals
#[derive(Clone, Default)]
pub struct FormState {
    /// Individual field signals - each field manages its own reactive state
    pub fields: HashMap<String, FieldSignal>,
    /// Current errors for each field
    pub errors: HashMap<String, ValidationError>,
    /// Touched state for each field
    pub touched: HashMap<String, bool>,
    /// Initial values for each field (to track dirty state)
    pub initial_values: HashMap<String, String>,
}

impl FormState {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            errors: HashMap::new(),
            touched: HashMap::new(),
            initial_values: HashMap::new(),
        }
    }

    /// Get or create a field signal
    pub fn get_or_create_field(&mut self, name: &str) -> FieldSignal {
        let field = self
            .fields
            .entry(name.to_string())
            .or_insert_with(|| FieldSignal {
                value: RwSignal::new(String::new()),
            })
            .clone();

        // Store initial value if not already stored
        self.initial_values.entry(name.to_string()).or_default();

        field
    }

    /// Check if a specific field is dirty (value differs from initial value)
    pub fn is_field_dirty(&self, name: &str) -> bool {
        if let (Some(field), Some(initial)) = (self.fields.get(name), self.initial_values.get(name))
        {
            field.value.get_untracked() != *initial
        } else {
            false
        }
    }

    /// Check if any field is dirty
    pub fn is_form_dirty(&self) -> bool {
        self.fields.iter().any(|(name, field)| {
            if let Some(initial) = self.initial_values.get(name) {
                field.value.get_untracked() != *initial
            } else {
                false
            }
        })
    }

    /// Check if a specific field is touched
    pub fn is_field_touched(&self, name: &str) -> bool {
        self.touched.get(name).copied().unwrap_or(false)
    }

    /// Check if any field is touched
    pub fn is_form_touched(&self) -> bool {
        self.touched.values().any(|&touched| touched)
    }
}

/// The main form handle that users interact with
#[derive(Clone)]
pub struct Form<T: FormValidation> {
    state: RwSignal<FormState>,
    /// Store the form data instance for validation
    form_data: RwSignal<T>,
}

// Manually implement Copy for Form<T> regardless of whether T is Copy
// This is safe because Form only contains Copy types (RwSignal is always Copy)
impl<T: FormValidation + Clone> Copy for Form<T> {}

impl<T: FormValidation + Default + Clone + Send + Sync + 'static> Form<T> {
    pub fn new() -> Self {
        Self {
            state: RwSignal::new(FormState::new()),
            form_data: RwSignal::new(T::default()),
        }
    }

    /// Create a new form initialized with data from an existing instance
    ///
    /// This method allows you to populate a form with existing data, converting
    /// the typed fields to strings for the form inputs.
    ///
    /// # Example
    /// ```rust,ignore
    /// let contact = Contact {
    ///     name: "Jed".to_string(),
    ///     email: "jed@email.com".to_string(),
    /// };
    /// let form = Form::from(contact);
    /// ```
    pub fn from(data: T) -> Self {
        let string_values = data.to_strings();
        let mut form_state = FormState::new();

        // Initialize field signals with the string values
        for (field_name, value) in string_values.iter() {
            let field_signal = form_state.get_or_create_field(field_name);
            field_signal.value.set(value.clone());
            // Set initial value for dirty tracking
            form_state
                .initial_values
                .insert(field_name.clone(), value.clone());
        }

        Self {
            state: RwSignal::new(form_state),
            form_data: RwSignal::new(data),
        }
    }

    /// Validate all fields using the form struct's validation
    pub fn validate(&self) -> bool {
        // Sync current field values to form_data
        let parse_errors = self.sync_to_form_data();

        // If there were parse errors, add them to state and return false
        if !parse_errors.is_empty() {
            self.state.update(|state| {
                state.errors.extend(parse_errors);
            });
            return false;
        }

        // Run validation on form_data
        let validation_errors = self.form_data.get_untracked().validate_all();

        // Update state with errors
        self.state.update(|state| {
            state.errors = validation_errors;
        });

        // Return true if no errors
        self.state.get_untracked().errors.is_empty()
    }

    /// Sync field values from signals to the form data struct
    /// Returns parse errors for fields that couldn't be converted
    fn sync_to_form_data(&self) -> HashMap<String, ValidationError> {
        let state = self.state.get_untracked();
        self.form_data.update_untracked(|data| {
            // This will be implemented by the derive macro
            // to parse strings into appropriate types
            data.sync_from_strings(&state.fields)
        })
    }

    /// Reset all form values and errors
    pub fn reset(&self) {
        self.state.update(|state| {
            for field in state.fields.values() {
                field.value.set(String::new());
            }
            state.errors.clear();
            state.touched.clear();
            // Reset initial values to empty strings
            for initial in state.initial_values.values_mut() {
                *initial = String::new();
            }
        });
        self.form_data.set(T::default());
    }

    /// Get current form values as a map of strings
    pub fn values(&self) -> HashMap<String, String> {
        let state = self.state.get_untracked();
        state
            .fields
            .iter()
            .map(|(name, field)| (name.clone(), field.value.get_untracked()))
            .collect()
    }

    /// Get the typed form data (after validation)
    pub fn data(&self) -> T {
        self.form_data.get_untracked()
    }

    /// Get the internal state signal
    pub(crate) fn state_signal(&self) -> RwSignal<FormState> {
        self.state
    }

    /// Validate a single field by name
    ///
    /// This method is called automatically by the Field component when a field value changes,
    /// enabling reactive validation. It:
    /// 1. Syncs the field value from the signal to the form data struct
    /// 2. Checks for parse errors (e.g., invalid number format)
    /// 3. Runs the field's validation rules
    /// 4. Updates the form state errors reactively
    ///
    /// The error signals in Field components automatically update when this method
    /// modifies the form state, providing immediate feedback to users.
    pub fn validate_field(&self, field_name: &str) {
        // First, sync the specific field value to form_data
        let state = self.state.get_untracked();

        // Sync the field value to form_data and check for parse errors
        if let Some(field_signal) = state.fields.get(field_name) {
            let _value = field_signal.value.get_untracked();

            // Try to parse the value using the form data's sync method
            // We need to create a temporary map with just this field
            let mut temp_fields = HashMap::new();
            temp_fields.insert(field_name.to_string(), field_signal.clone());

            let parse_errors = self
                .form_data
                .update_untracked(|data| data.sync_from_strings(&temp_fields));

            // If there was a parse error, set it and return
            if let Some(parse_error) = parse_errors.get(field_name) {
                self.state.update(|state| {
                    state
                        .errors
                        .insert(field_name.to_string(), parse_error.clone());
                });
                return;
            }

            // No parse error, run field validation
            let validation_result = self.form_data.get_untracked().validate_field(field_name);

            // Update the error state based on validation result
            self.state.update(|state| {
                match validation_result {
                    Ok(()) => {
                        // Validation passed, remove any existing error
                        state.errors.remove(field_name);
                    }
                    Err(err) => {
                        // Validation failed, set the error
                        state.errors.insert(field_name.to_string(), err);
                    }
                }
            });
        }
    }
}

impl<T: FormValidation + Default + Clone + Send + Sync + 'static> Default for Form<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Form component that provides form state to children via context
///
/// # Example
/// ```rust,ignore
/// view! {
///     <FormComponent form=form let:form_state>
///         // Access form state properties
///         <div>{move || format!("Values: {:?}", form_state.values.get())}</div>
///         <div>{move || format!("Form is dirty: {}", form_state.dirty.get())}</div>
///     </FormComponent>
/// }
/// ```
#[component]
pub fn FormComponent<T, F, IV>(form: Form<T>, children: F) -> impl IntoView
where
    T: FormValidation + Default + Clone + Send + Sync + 'static,
    F: Fn(FormComponentState) -> IV + 'static,
    IV: IntoView,
{
    // Provide form context to children
    provide_context(form);

    // Create derived signals for form values and errors
    let form_values = Signal::derive(move || form.values());

    let form_errors = Signal::derive({
        move || {
            let state = form.state_signal().get();
            state.errors.clone()
        }
    });

    // Create derived signal for form dirty state
    let form_dirty = Signal::derive({
        move || {
            let state = form.state_signal().get();
            state.is_form_dirty()
        }
    });

    // Create derived signal for form touched state
    let form_touched = Signal::derive({
        move || {
            let state = form.state_signal().get();
            state.is_form_touched()
        }
    });

    // Create derived signal for form valid state
    let form_valid = Signal::derive({
        move || {
            let state = form.state_signal().get();
            state.errors.is_empty()
        }
    });

    // Create FormComponentState object
    let form_state = FormComponentState {
        values: form_values,
        errors: form_errors,
        dirty: form_dirty,
        touched: form_touched,
        valid: form_valid,
    };

    // Pass state to children via the children function
    // This enables the `let:form_state` syntax
    children(form_state)
}

impl<T: FormValidation + Default + Clone + Send + Sync + 'static> From<T> for Form<T> {
    fn from(data: T) -> Self {
        Self::from(data)
    }
}
