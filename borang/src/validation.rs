use leptos::prelude::*;
use std::collections::HashMap;

/// A field's reactive value signal.
///
/// This struct wraps a Leptos `RwSignal<String>` to provide reactive updates
/// for form field values. It's used internally by the form system to manage
/// individual field state.
#[derive(Clone)]
pub struct FieldSignal {
    /// The reactive signal containing the field's string value
    pub value: RwSignal<String>,
}

/// Represents the kind of validation error that occurred.
///
/// This enum categorizes validation errors and stores their parameters,
/// allowing for i18n-friendly error message generation.
///
/// # Example
///
/// ```rust,ignore
/// use borang::ErrorKind;
///
/// let error = ErrorKind::Required {
///     field: "email".to_string(),
/// };
/// ```
#[derive(Clone, Debug)]
pub enum ErrorKind {
    /// Field is required but empty
    Required { field: String },
    /// Email format is invalid
    InvalidEmail { field: String },
    /// String length constraints not met
    InvalidLength {
        field: String,
        min: Option<usize>,
        max: Option<usize>,
    },
    /// Numeric range constraints not met
    InvalidRange {
        field: String,
        min: Option<i64>,
        max: Option<i64>,
    },
    /// Parse error (e.g., invalid number)
    ParseError {
        field: String,
        expected_type: String,
    },
    /// Custom validation error
    Custom { field: String, message: String },
}

impl ErrorKind {
    /// Get the field name for this error.
    pub fn field(&self) -> &str {
        match self {
            ErrorKind::Required { field } => field,
            ErrorKind::InvalidEmail { field } => field,
            ErrorKind::InvalidLength { field, .. } => field,
            ErrorKind::InvalidRange { field, .. } => field,
            ErrorKind::ParseError { field, .. } => field,
            ErrorKind::Custom { field, .. } => field,
        }
    }

    /// Get the default English error message (for backward compatibility).
    pub fn default_message(&self) -> String {
        match self {
            ErrorKind::Required { field } => format!("{} is required", field),
            ErrorKind::InvalidEmail { field } => format!("{} must be a valid email address", field),
            ErrorKind::InvalidLength {
                field,
                min: Some(min),
                max: Some(max),
            } => {
                format!("{} must be between {} and {} characters", field, min, max)
            }
            ErrorKind::InvalidLength {
                field,
                min: Some(min),
                max: None,
            } => {
                format!("{} must be at least {} characters", field, min)
            }
            ErrorKind::InvalidLength {
                field,
                min: None,
                max: Some(max),
            } => {
                format!("{} must be at most {} characters", field, max)
            }
            ErrorKind::InvalidLength { field, .. } => format!("{} has invalid length", field),
            ErrorKind::InvalidRange {
                field,
                min: Some(min),
                max: Some(max),
            } => {
                format!("{} must be between {} and {}", field, min, max)
            }
            ErrorKind::InvalidRange {
                field,
                min: Some(min),
                max: None,
            } => {
                format!("{} must be at least {}", field, min)
            }
            ErrorKind::InvalidRange {
                field,
                min: None,
                max: Some(max),
            } => {
                format!("{} must be at most {}", field, max)
            }
            ErrorKind::InvalidRange { field, .. } => format!("{} is out of range", field),
            ErrorKind::ParseError {
                field,
                expected_type,
            } => {
                format!("{} must be a valid {}", field, expected_type)
            }
            ErrorKind::Custom { message, .. } => message.clone(),
        }
    }
}

/// Represents a validation error for a specific field.
///
/// This type is returned when validation fails, containing both the field name
/// and a human-readable error message.
///
/// # Example
///
/// ```rust,ignore
/// use borang::ValidationError;
///
/// let error = ValidationError::new("email", "Email is required");
/// assert_eq!(error.message(), "Email is required");
/// ```
#[derive(Clone, Debug)]
pub struct ValidationError {
    /// The name of the field that failed validation
    pub field: String,
    /// The human-readable error message (for backward compatibility)
    pub message: String,
    /// The structured error kind (for i18n)
    pub kind: ErrorKind,
}

impl ValidationError {
    /// Create a new validation error with a message (legacy API).
    ///
    /// # Parameters
    ///
    /// - `field`: The name of the field that failed validation
    /// - `message`: A human-readable error message
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        let field = field.into();
        let message = message.into();
        Self {
            field: field.clone(),
            message: message.clone(),
            kind: ErrorKind::Custom { field, message },
        }
    }

    /// Create a new validation error with an error kind.
    ///
    /// # Parameters
    ///
    /// - `kind`: The error kind containing structured error information
    pub fn with_kind(kind: ErrorKind) -> Self {
        let field = kind.field().to_string();
        let message = kind.default_message();
        Self {
            field,
            message,
            kind,
        }
    }

    /// Get the error message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the error kind.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// Translate the error message using a provided translator function.
    ///
    /// This allows you to provide custom i18n logic without coupling
    /// the validation library to any specific i18n framework.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let translated = error.translate(|kind| {
    ///     match kind {
    ///         ErrorKind::Required { field } => format!("{} es requerido", field),
    ///         _ => kind.default_message(),
    ///     }
    /// });
    /// ```
    pub fn translate<F>(&self, translator: F) -> String
    where
        F: FnOnce(&ErrorKind) -> String,
    {
        translator(&self.kind)
    }
}

/// Result type for validation operations.
///
/// Returns `Ok(())` if validation succeeds, or `Err(ValidationError)` if it fails.
pub type ValidationResult = Result<(), ValidationError>;

/// Trait for types that can be parsed from form field strings.
///
/// This trait enables type-safe conversion between HTML form input values (strings)
/// and Rust types. It's used internally by the form system to convert user input
/// into typed form data during validation.
///
/// # Built-in Implementations
///
/// The following types have built-in implementations:
///
/// - `String` - Direct pass-through
/// - `i32`, `i64`, `u32`, `u64` - Integer parsing
/// - `f32`, `f64` - Floating-point parsing
/// - `bool` - Checkbox/toggle support ("on", "true", "1" = true)
///
/// # Custom Types
///
/// You can implement this trait for your own types to use them in forms.
/// This is particularly useful for enums, newtypes, and domain-specific types.
///
/// ## Example: Custom Enum
///
/// ```rust,ignore
/// use borang::{FromFieldValue, ValidationError};
///
/// #[derive(Clone, Debug)]
/// enum Country {
///     US,
///     UK,
///     Canada,
///     Other,
/// }
///
/// impl FromFieldValue for Country {
///     fn from_field_value(field_name: &str, value: &str) -> Result<Self, ValidationError> {
///         match value {
///             "us" => Ok(Country::US),
///             "uk" => Ok(Country::UK),
///             "canada" => Ok(Country::Canada),
///             "other" => Ok(Country::Other),
///             _ => Err(ValidationError::new(field_name, "Invalid country selection")),
///         }
///     }
///
///     fn to_field_value(&self) -> String {
///         match self {
///             Country::US => "us",
///             Country::UK => "uk",
///             Country::Canada => "canada",
///             Country::Other => "other",
///         }.to_string()
///     }
/// }
/// ```
///
/// ## Example: Newtype with Validation
///
/// ```rust,ignore
/// use borang::{FromFieldValue, ValidationError};
///
/// #[derive(Clone, Debug)]
/// struct Email(String);
///
/// impl FromFieldValue for Email {
///     fn from_field_value(field_name: &str, value: &str) -> Result<Self, ValidationError> {
///         if value.contains('@') && value.contains('.') {
///             Ok(Email(value.to_string()))
///         } else {
///             Err(ValidationError::new(field_name, "Invalid email format"))
///         }
///     }
///
///     fn to_field_value(&self) -> String {
///         self.0.clone()
///     }
/// }
/// ```
///
/// ## Example: Optional Fields
///
/// ```rust,ignore
/// use borang::{FromFieldValue, ValidationError};
///
/// impl<T: FromFieldValue> FromFieldValue for Option<T> {
///     fn from_field_value(field_name: &str, value: &str) -> Result<Self, ValidationError> {
///         if value.trim().is_empty() {
///             Ok(None)
///         } else {
///             T::from_field_value(field_name, value).map(Some)
///         }
///     }
///
///     fn to_field_value(&self) -> String {
///         match self {
///             Some(v) => v.to_field_value(),
///             None => String::new(),
///         }
///     }
/// }
/// ```
///
/// # Error Handling
///
/// Parse errors are treated as validation errors and appear in the form's error state
/// alongside other validation errors. This provides immediate feedback to users when
/// they enter invalid data (e.g., "abc" in a number field).
///
/// # Usage in Forms
///
/// Once implemented, you can use your custom type in form structs:
///
/// ```rust,ignore
/// use borang::FormValidation;
///
/// #[derive(FormValidation, Default, Clone)]
/// struct UserForm {
///     #[validator(required)]
///     name: String,
///
///     #[validator(required)]
///     country: Country,  // Your custom type
///
///     email: Email,  // Another custom type
/// }
/// ```
pub trait FromFieldValue: Sized {
    /// Parse a string value from a form field into this type.
    ///
    /// # Parameters
    ///
    /// - `field_name`: The name of the field being parsed (for error messages)
    /// - `value`: The string value from the HTML input
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` if parsing succeeds
    /// - `Err(ValidationError)` if parsing fails
    fn from_field_value(field_name: &str, value: &str) -> Result<Self, ValidationError>;

    /// Convert this type back to a string for display in form fields.
    ///
    /// This is used when populating form fields with existing data.
    fn to_field_value(&self) -> String;
}

// Implement for String
impl FromFieldValue for String {
    fn from_field_value(_field_name: &str, value: &str) -> Result<Self, ValidationError> {
        Ok(value.to_string())
    }

    fn to_field_value(&self) -> String {
        self.clone()
    }
}

// Implement for i32
impl FromFieldValue for i32 {
    fn from_field_value(field_name: &str, value: &str) -> Result<Self, ValidationError> {
        value.parse().map_err(|_| {
            ValidationError::with_kind(ErrorKind::ParseError {
                field: field_name.to_string(),
                expected_type: "number".to_string(),
            })
        })
    }

    fn to_field_value(&self) -> String {
        self.to_string()
    }
}

// Implement for i64
impl FromFieldValue for i64 {
    fn from_field_value(field_name: &str, value: &str) -> Result<Self, ValidationError> {
        value.parse().map_err(|_| {
            ValidationError::with_kind(ErrorKind::ParseError {
                field: field_name.to_string(),
                expected_type: "number".to_string(),
            })
        })
    }

    fn to_field_value(&self) -> String {
        self.to_string()
    }
}

// Implement for u32
impl FromFieldValue for u32 {
    fn from_field_value(field_name: &str, value: &str) -> Result<Self, ValidationError> {
        value.parse().map_err(|_| {
            ValidationError::with_kind(ErrorKind::ParseError {
                field: field_name.to_string(),
                expected_type: "number".to_string(),
            })
        })
    }

    fn to_field_value(&self) -> String {
        self.to_string()
    }
}

// Implement for u64
impl FromFieldValue for u64 {
    fn from_field_value(field_name: &str, value: &str) -> Result<Self, ValidationError> {
        value.parse().map_err(|_| {
            ValidationError::with_kind(ErrorKind::ParseError {
                field: field_name.to_string(),
                expected_type: "number".to_string(),
            })
        })
    }

    fn to_field_value(&self) -> String {
        self.to_string()
    }
}

// Implement for f32
impl FromFieldValue for f32 {
    fn from_field_value(field_name: &str, value: &str) -> Result<Self, ValidationError> {
        value.parse().map_err(|_| {
            ValidationError::with_kind(ErrorKind::ParseError {
                field: field_name.to_string(),
                expected_type: "number".to_string(),
            })
        })
    }

    fn to_field_value(&self) -> String {
        self.to_string()
    }
}

// Implement for f64
impl FromFieldValue for f64 {
    fn from_field_value(field_name: &str, value: &str) -> Result<Self, ValidationError> {
        value.parse().map_err(|_| {
            ValidationError::with_kind(ErrorKind::ParseError {
                field: field_name.to_string(),
                expected_type: "number".to_string(),
            })
        })
    }

    fn to_field_value(&self) -> String {
        self.to_string()
    }
}

// Implement for bool
impl FromFieldValue for bool {
    fn from_field_value(_field_name: &str, value: &str) -> Result<Self, ValidationError> {
        // For checkboxes: "on" or "true" = true, anything else = false
        Ok(value == "on" || value == "true" || value == "1")
    }

    fn to_field_value(&self) -> String {
        if *self { "true" } else { "false" }.to_string()
    }
}

/// Trait that form structs implement (via derive macro).
///
/// This trait is automatically implemented when you use `#[derive(FormValidation)]`
/// on your form struct. You typically don't need to implement this trait manually.
///
/// # Example
///
/// ```rust,ignore
/// use borang::FormValidation;
///
/// #[derive(FormValidation, Default, Clone)]
/// struct LoginForm {
///     #[validator(required, email)]
///     email: String,
///
///     #[validator(required, length(min = 8))]
///     password: String,
/// }
///
/// // The derive macro generates the FormValidation implementation
/// // You can now use this struct with Form<LoginForm>
/// ```
///
/// # Generated Methods
///
/// The derive macro generates implementations for all trait methods based on
/// the `#[validator(...)]` attributes on your struct fields.
pub trait FormValidation {
    /// Validate all fields and return a map of field names to errors.
    ///
    /// This method runs validation for every field in the form and collects
    /// any errors that occur. If a field has multiple validators, only the
    /// first error is returned (first error wins).
    ///
    /// # Returns
    ///
    /// A `HashMap` where keys are field names and values are validation errors.
    /// An empty map indicates all fields are valid.
    fn validate_all(&self) -> HashMap<String, ValidationError>;

    /// Validate a specific field by name.
    ///
    /// This method runs validation for a single field, executing all validators
    /// defined for that field in order.
    ///
    /// # Parameters
    ///
    /// - `field_name`: The name of the field to validate
    ///
    /// # Returns
    ///
    /// - `Ok(())` if validation succeeds
    /// - `Err(ValidationError)` if validation fails
    fn validate_field(&self, field_name: &str) -> ValidationResult;

    /// Get all field names defined in the form.
    ///
    /// # Returns
    ///
    /// A vector of static string slices containing all field names.
    fn field_names() -> Vec<&'static str>;

    /// Sync field values from string map (called by Form).
    ///
    /// This method is called internally by the form system to convert string
    /// values from HTML inputs into the typed fields of the struct. It uses
    /// the `FromFieldValue` trait for conversion.
    ///
    /// # Parameters
    ///
    /// - `fields`: A map of field names to their reactive signals
    ///
    /// # Returns
    ///
    /// A map of field names to parse errors. An empty map indicates all
    /// fields were successfully parsed.
    fn sync_from_strings(
        &mut self,
        fields: &HashMap<String, FieldSignal>,
    ) -> HashMap<String, ValidationError>;

    /// Convert form data to string map (for display).
    ///
    /// This method converts all typed field values back to strings using
    /// the `FromFieldValue::to_field_value` method. It's used when populating
    /// form fields with existing data.
    ///
    /// # Returns
    ///
    /// A map of field names to their string representations.
    fn to_strings(&self) -> HashMap<String, String>;
}
