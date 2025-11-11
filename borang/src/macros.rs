/// Re-export of the FormValidation derive macro from borang-macros.
///
/// This macro generates the `FormValidation` trait implementation for a struct,
/// parsing `#[validator(...)]` attributes on fields to generate validation logic.
///
/// # Example
///
/// ```ignore
/// use borang::FormValidation;
///
/// #[derive(FormValidation, Default, Clone)]
/// pub struct SignUpForm {
///     #[validator(required)]
///     name: String,
///
///     #[validator(required, email)]
///     email: String,
///
///     #[validator(required, length(min = 8))]
///     password: String,
/// }
/// ```
pub use borang_macros::FormValidation;
