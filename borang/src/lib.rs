pub mod field;
pub mod input;
pub mod rules;

pub use field::{Field, FieldError, FieldErrorFor};
pub use input::Input;
pub use rules::WithMessage;

use std::{collections::BTreeMap, fmt::Display, sync::Arc};

use leptos::prelude::*;

#[derive(Clone)]
pub struct UseFormValidation {
    pub form: RwSignal<FormValidation>,
    validate_trigger: RwSignal<u32>,
    touch_trigger: RwSignal<u32>,
    field_count: RwSignal<usize>,
}

impl UseFormValidation {
    pub fn new() -> Self {
        Self {
            form: RwSignal::new(FormValidation::new()),
            validate_trigger: RwSignal::new(0),
            touch_trigger: RwSignal::new(0),
            field_count: RwSignal::new(0),
        }
    }

    pub fn field<T>(&self, field_name: &str, initial_value: T) -> ValidatedField<T>
    where
        T: Clone + Send + Sync + 'static,
    {
        let field = ValidatedField::new(self.form, initial_value, field_name);

        self.field_count.update(|count| *count += 1);

        let validate_trigger = self.validate_trigger;
        let touch_trigger = self.touch_trigger;

        {
            let field = field.clone();
            Effect::new(move |prev_trigger: Option<u32>| {
                let current_trigger = validate_trigger.get();

                if let Some(prev) = prev_trigger {
                    if current_trigger != prev {
                        field.validate();
                    }
                }

                current_trigger
            });
        }

        {
            let field = field.clone();
            Effect::new(move |prev_trigger| {
                let current_trigger = touch_trigger.get();

                if let Some(prev) = prev_trigger {
                    if current_trigger != prev {
                        field.mark_touched();
                    }
                }

                current_trigger
            });
        }
        field
    }

    pub fn is_dirty(&self) -> bool {
        self.form
            .get_untracked()
            .fields
            .iter()
            .any(|(_, field)| field.is_dirty)
    }

    pub fn is_touched(&self) -> bool {
        self.form
            .get_untracked()
            .fields
            .iter()
            .any(|(_, field)| field.is_touched)
    }

    /// Validates all registered fields by triggering the validation signal
    pub fn validate_all_fields(&self) -> bool {
        // Increment the trigger to cause all fields to validate
        self.validate_trigger.update(|val| *val += 1);

        let is_valid = self.form.get_untracked().is_valid;

        // Return the validation result
        is_valid
    }

    /// Touches all registered fields by triggering the touch signal
    pub fn touch_all_fields(&self) {
        self.touch_trigger.update(|val| *val += 1);
    }
}

#[derive(Debug, Clone)]
pub struct FormValidation {
    pub fields: BTreeMap<String, FieldState>,
    pub is_valid: bool,
}

impl FormValidation {
    pub fn new() -> Self {
        Self {
            fields: BTreeMap::new(),
            is_valid: true,
        }
    }

    pub fn get_field_error(&self, field_name: &str) -> Option<&ValidationError> {
        self.fields
            .get(field_name)
            .and_then(|state| state.error.as_ref())
    }

    pub fn should_show_error(&self, field_name: &str) -> bool {
        if let Some(state) = self.fields.get(field_name) {
            state.is_touched && state.error.is_some()
        } else {
            false
        }
    }

    pub fn update_field_state<F>(&mut self, field_name: &str, updater: F)
    where
        F: FnOnce(&mut FieldState),
    {
        let state = self
            .fields
            .entry(field_name.to_string())
            .or_insert_with(|| FieldState {
                is_dirty: false,
                is_touched: false,
                error: None,
            });
        updater(state);
        self.update_form_validity();
    }

    fn update_form_validity(&mut self) {
        self.is_valid = self.fields.values().all(|state| state.error.is_none());
    }
}

#[derive(Debug, Clone)]
pub struct FieldState {
    pub is_dirty: bool,
    pub is_touched: bool,
    pub error: Option<ValidationError>,
}

pub struct ValidatedField<T> {
    pub form: RwSignal<FormValidation>,
    pub value: RwSignal<T>,
    pub field_name: String,
    pub validator: Option<Arc<dyn ValidationRule<T>>>,
}

impl<T> Clone for ValidatedField<T> {
    fn clone(&self) -> Self {
        Self {
            form: self.form,
            value: self.value,
            field_name: self.field_name.clone(),
            validator: self.validator.clone(),
        }
    }
}

impl<T> ValidatedField<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new(form: RwSignal<FormValidation>, value: T, field_name: &str) -> Self {
        Self {
            form,
            value: RwSignal::new(value),
            field_name: field_name.to_string(),
            validator: None,
        }
    }

    pub fn with_validator<V: ValidationRule<T> + 'static>(mut self, validator: V) -> Self {
        self.validator = Some(Arc::new(validator));
        self.validate();
        self
    }

    pub fn validate(&self) -> bool {
        if let Some(validator) = &self.validator {
            let value = self.value.get_untracked();

            let result = validator.validate(&self.field_name, &value);
            let is_valid = result.is_ok();

            self.form.update(|form| {
                form.update_field_state(&self.field_name, |state| match result {
                    Ok(_) => state.error = None,
                    Err(err) => state.error = Some(err),
                });
            });

            is_valid
        } else {
            true
        }
    }

    pub fn mark_touched(&self) {
        self.form.update(|form| {
            form.update_field_state(&self.field_name, |state| {
                state.is_touched = true;
            });
        });
    }

    pub fn mark_dirty(&self) {
        self.form.update(|form| {
            form.update_field_state(&self.field_name, |state| {
                state.is_dirty = true;
            });
        });
    }

    pub fn error_message(&self) -> Signal<Option<String>> {
        let form = self.form;
        let field_name = self.field_name.clone();

        Signal::derive(move || {
            let form = form.get();

            if form.should_show_error(&field_name) {
                form.get_field_error(&field_name)
                    .map(|error| error.message())
            } else {
                None
            }
        })
    }
}

#[derive(Clone)]
pub struct ValidationError {
    pub field: String,
    message_fn: Arc<dyn Fn() -> String + Send + Sync>,
}

impl ValidationError {
    pub fn new(field: String, message_fn: impl Fn() -> String + Send + Sync + 'static) -> Self {
        Self {
            field,
            message_fn: Arc::new(message_fn),
        }
    }

    pub fn message(&self) -> String {
        (self.message_fn)()
    }
}

impl std::fmt::Debug for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValidationError")
            .field("field", &self.field)
            .field("message", &self.message())
            .finish()
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

pub type ValidationResult = Result<(), ValidationError>;

pub trait ValidationRule<T>: Send + Sync {
    fn validate(&self, field_name: &str, value: &T) -> ValidationResult;
}
