use borang::{ErrorKind, ValidationError};
use leptos_i18n::I18nContext;

use crate::i18n::*;

/// Translate a type name to the current locale.
fn translate_type_name(i18n: I18nContext<Locale>, type_name: &str) -> String {
    match type_name {
        "number" => t_string!(i18n, type_number).to_string(),
        "email" => t_string!(i18n, type_email).to_string(),
        "boolean" => t_string!(i18n, type_boolean).to_string(),
        _ => type_name.to_string(),
    }
}

/// Translate a validation error using the current locale.
///
/// This function takes a validation error and translates it based on the
/// error kind and the current i18n context. The caller provides the translated
/// field name, giving full control over field name translation.
///
/// # Example
///
/// ```rust,ignore
/// let i18n = use_i18n();
/// let error = ValidationError::with_kind(ErrorKind::Required {
///     field: "name".to_string(),
/// });
/// let field_name = t_string!(i18n, name);
/// let translated = translate_validation_error(i18n, &error, &field_name);
/// ```
pub fn translate_validation_error(
    i18n: I18nContext<Locale>,
    error: &ValidationError,
    field_name: &str,
) -> String {
    match error.kind() {
        ErrorKind::Required { .. } => {
            t_string!(i18n, item_is_required, item = field_name).to_string()
        }
        ErrorKind::InvalidEmail { .. } => {
            t_string!(i18n, item_is_not_valid, item = field_name).to_string()
        }
        ErrorKind::InvalidLength {
            min: Some(min),
            max: Some(max),
            ..
        } => t_string!(
            i18n,
            item_must_be_between,
            item = field_name,
            min = min,
            max = max
        )
        .to_string(),
        ErrorKind::InvalidLength {
            min: Some(min),
            max: None,
            ..
        } => t_string!(
            i18n,
            item_must_be_at_least_chars,
            item = field_name,
            min = min
        )
        .to_string(),
        ErrorKind::InvalidLength {
            min: None,
            max: Some(max),
            ..
        } => t_string!(
            i18n,
            item_must_be_at_most_chars,
            item = field_name,
            max = max
        )
        .to_string(),
        ErrorKind::InvalidLength { .. } => {
            t_string!(i18n, item_is_not_valid, item = field_name).to_string()
        }
        ErrorKind::InvalidRange {
            min: Some(min),
            max: Some(max),
            ..
        } => t_string!(
            i18n,
            item_must_be_between,
            item = field_name,
            min = min,
            max = max
        )
        .to_string(),
        ErrorKind::InvalidRange {
            min: Some(min),
            max: None,
            ..
        } => t_string!(i18n, item_must_be_at_least, item = field_name, min = min).to_string(),
        ErrorKind::InvalidRange {
            min: None,
            max: Some(max),
            ..
        } => t_string!(i18n, item_must_be_at_most, item = field_name, max = max).to_string(),
        ErrorKind::InvalidRange { .. } => {
            t_string!(i18n, item_is_not_valid, item = field_name).to_string()
        }
        ErrorKind::ParseError { expected_type, .. } => {
            let translated_type = translate_type_name(i18n, expected_type);
            t_string!(
                i18n,
                item_must_be_valid_type,
                item = field_name,
                value_type = &translated_type
            )
            .to_string()
        }
        ErrorKind::Custom { message, .. } => message.clone(),
    }
}
