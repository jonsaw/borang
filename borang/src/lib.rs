//! # Borang API

pub mod field;
pub mod form;
pub mod macros;
pub mod validation;

// Re-export core types
pub use field::{Field, FieldState, GetField};
pub use form::{Form, FormComponent, FormComponentState, FormState};
pub use macros::FormValidation as Validation;
pub use validation::{
    ErrorKind, FieldSignal, FormValidation, FromFieldValue, ValidationError, ValidationResult,
};
