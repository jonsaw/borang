use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Expr, ExprLit, Field, Fields, Lit, Meta, Token, parse::Parse,
    parse::ParseStream, parse_macro_input,
};

/// Internal representation of a validator and its parameters
#[derive(Debug, Clone)]
enum Validator {
    Required,
    Email,
    Length {
        min: Option<usize>,
        max: Option<usize>,
    },
    Range {
        min: Option<i64>,
        max: Option<i64>,
    },
    Custom {
        method_name: String,
    },
}

/// Represents all validation rules for a single field
struct FieldValidation {
    field_name: String,
    field_type: syn::Type,
    validators: Vec<Validator>,
}

/// Parse validator parameters like `min = 8, max = 100`
struct ValidatorParams {
    params: Vec<(String, syn::Lit)>,
}

impl Parse for ValidatorParams {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut params = Vec::new();

        while !input.is_empty() {
            let name: syn::Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let value: syn::Lit = input.parse()?;
            params.push((name.to_string(), value));

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(ValidatorParams { params })
    }
}

/// Parse a single validator from attribute content
fn parse_validator(meta: &Meta) -> syn::Result<Validator> {
    match meta {
        // Simple validators: #[validator(required)]
        Meta::Path(path) => {
            let ident = path
                .get_ident()
                .ok_or_else(|| syn::Error::new_spanned(path, "Expected validator name"))?;

            match ident.to_string().as_str() {
                "required" => Ok(Validator::Required),
                "email" => Ok(Validator::Email),
                name => Err(syn::Error::new_spanned(
                    ident,
                    format!(
                        "Unknown validator: '{}'. Valid validators are: required, email, length, range, custom",
                        name
                    ),
                )),
            }
        }

        // Validators with parameters: #[validator(length(min = 8))]
        Meta::List(list) => {
            let validator_name = list
                .path
                .get_ident()
                .ok_or_else(|| syn::Error::new_spanned(&list.path, "Expected validator name"))?
                .to_string();

            match validator_name.as_str() {
                "length" => {
                    let params: ValidatorParams = syn::parse2(list.tokens.clone())?;
                    let mut min = None;
                    let mut max = None;

                    for (name, value) in params.params {
                        match name.as_str() {
                            "min" => {
                                if let syn::Lit::Int(lit_int) = value {
                                    min = Some(lit_int.base10_parse::<usize>().map_err(|e| {
                                        syn::Error::new_spanned(
                                            lit_int,
                                            format!("Invalid min value: {}", e),
                                        )
                                    })?);
                                } else {
                                    return Err(syn::Error::new_spanned(
                                        value,
                                        "min parameter must be an integer",
                                    ));
                                }
                            }
                            "max" => {
                                if let syn::Lit::Int(lit_int) = value {
                                    max = Some(lit_int.base10_parse::<usize>().map_err(|e| {
                                        syn::Error::new_spanned(
                                            lit_int,
                                            format!("Invalid max value: {}", e),
                                        )
                                    })?);
                                } else {
                                    return Err(syn::Error::new_spanned(
                                        value,
                                        "max parameter must be an integer",
                                    ));
                                }
                            }
                            _ => {
                                return Err(syn::Error::new_spanned(
                                    &list.tokens,
                                    format!(
                                        "Unknown parameter '{}' for length validator. Valid parameters: min, max",
                                        name
                                    ),
                                ));
                            }
                        }
                    }

                    if min.is_none() && max.is_none() {
                        return Err(syn::Error::new_spanned(
                            &list.tokens,
                            "length validator requires at least one of: min, max",
                        ));
                    }

                    Ok(Validator::Length { min, max })
                }

                "range" => {
                    let params: ValidatorParams = syn::parse2(list.tokens.clone())?;
                    let mut min = None;
                    let mut max = None;

                    for (name, value) in params.params {
                        match name.as_str() {
                            "min" => {
                                if let syn::Lit::Int(lit_int) = value {
                                    min = Some(lit_int.base10_parse::<i64>().map_err(|e| {
                                        syn::Error::new_spanned(
                                            lit_int,
                                            format!("Invalid min value: {}", e),
                                        )
                                    })?);
                                } else {
                                    return Err(syn::Error::new_spanned(
                                        value,
                                        "min parameter must be an integer",
                                    ));
                                }
                            }
                            "max" => {
                                if let syn::Lit::Int(lit_int) = value {
                                    max = Some(lit_int.base10_parse::<i64>().map_err(|e| {
                                        syn::Error::new_spanned(
                                            lit_int,
                                            format!("Invalid max value: {}", e),
                                        )
                                    })?);
                                } else {
                                    return Err(syn::Error::new_spanned(
                                        value,
                                        "max parameter must be an integer",
                                    ));
                                }
                            }
                            _ => {
                                return Err(syn::Error::new_spanned(
                                    &list.tokens,
                                    format!(
                                        "Unknown parameter '{}' for range validator. Valid parameters: min, max",
                                        name
                                    ),
                                ));
                            }
                        }
                    }

                    if min.is_none() && max.is_none() {
                        return Err(syn::Error::new_spanned(
                            &list.tokens,
                            "range validator requires at least one of: min, max",
                        ));
                    }

                    Ok(Validator::Range { min, max })
                }

                "custom" => {
                    let params: ValidatorParams = syn::parse2(list.tokens.clone())?;

                    if params.params.len() != 1 {
                        return Err(syn::Error::new_spanned(
                            &list.tokens,
                            "custom validator requires exactly one parameter: the method name as a string",
                        ));
                    }

                    let (param_name, value) = &params.params[0];
                    if param_name != "method" && params.params.len() == 1 {
                        // Allow unnamed parameter for custom
                        if let syn::Lit::Str(lit_str) = value {
                            return Ok(Validator::Custom {
                                method_name: lit_str.value(),
                            });
                        }
                    }

                    return Err(syn::Error::new_spanned(
                        &list.tokens,
                        "custom validator parameter must be a string literal (e.g., custom(\"method_name\"))",
                    ));
                }

                name => Err(syn::Error::new_spanned(
                    &list.path,
                    format!(
                        "Unknown validator: '{}'. Valid validators are: required, email, length, range, custom",
                        name
                    ),
                )),
            }
        }

        // Name-value validators: #[validator(custom = "method_name")]
        Meta::NameValue(nv) => {
            let validator_name = nv
                .path
                .get_ident()
                .ok_or_else(|| syn::Error::new_spanned(&nv.path, "Expected validator name"))?
                .to_string();

            match validator_name.as_str() {
                "custom" => {
                    if let Expr::Lit(ExprLit {
                        lit: Lit::Str(lit_str),
                        ..
                    }) = &nv.value
                    {
                        Ok(Validator::Custom {
                            method_name: lit_str.value(),
                        })
                    } else {
                        Err(syn::Error::new_spanned(
                            &nv.value,
                            "custom validator value must be a string literal",
                        ))
                    }
                }
                name => Err(syn::Error::new_spanned(
                    &nv.path,
                    format!("Validator '{}' does not support name-value syntax", name),
                )),
            }
        }
    }
}

/// Parse all validators from a field's attributes
fn parse_field_validators(field: &Field) -> syn::Result<Vec<Validator>> {
    let mut validators = Vec::new();

    for attr in &field.attrs {
        // Only process #[validator(...)] attributes
        if !attr.path().is_ident("validator") {
            continue;
        }

        // Parse the attribute content
        // parse_nested_meta calls the closure once for each comma-separated item
        attr.parse_nested_meta(|meta| {
            // Convert ParseNestedMeta to Meta for parsing
            let path = meta.path.clone();

            // Check for parenthesized content (e.g., "length(min = 8)")
            if meta.input.peek(syn::token::Paren) {
                let content;
                syn::parenthesized!(content in meta.input);
                let tokens = content.parse::<proc_macro2::TokenStream>()?;

                let validator = parse_validator(&Meta::List(syn::MetaList {
                    path,
                    delimiter: syn::MacroDelimiter::Paren(Default::default()),
                    tokens,
                }))?;
                validators.push(validator);
                return Ok(());
            }

            // Check for = value (e.g., "custom = "method_name"")
            if meta.input.peek(Token![=]) {
                meta.input.parse::<Token![=]>()?;
                let value: Expr = meta.input.parse()?;

                let validator = parse_validator(&Meta::NameValue(syn::MetaNameValue {
                    path,
                    eq_token: Default::default(),
                    value,
                }))?;
                validators.push(validator);
                return Ok(());
            }

            // Otherwise it's a simple path (e.g., "required")
            // This must come last because we need to check for other patterns first
            let validator = parse_validator(&Meta::Path(path))?;
            validators.push(validator);
            Ok(())
        })?;
    }

    Ok(validators)
}

/// Extract field validation information from struct fields
fn extract_field_validations(data: &Data) -> syn::Result<Vec<FieldValidation>> {
    let fields = match data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    data_struct.fields.clone(),
                    "FormValidation can only be derived for structs with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "FormValidation can only be derived for structs",
            ));
        }
    };

    let mut field_validations = Vec::new();

    for field in fields {
        let field_name = field
            .ident
            .as_ref()
            .ok_or_else(|| syn::Error::new_spanned(field, "Field must have a name"))?
            .to_string();

        let validators = parse_field_validators(field)?;

        // Only include fields that have validators
        if !validators.is_empty() {
            field_validations.push(FieldValidation {
                field_name,
                field_type: field.ty.clone(),
                validators,
            });
        }
    }

    Ok(field_validations)
}

/// Generate validation code for a single validator
fn generate_validator_code(
    field_name: &str,
    _field_type: &syn::Type,
    validator: &Validator,
) -> proc_macro2::TokenStream {
    let field_ident = syn::Ident::new(field_name, proc_macro2::Span::call_site());

    match validator {
        Validator::Required => {
            quote! {
                // Required validation - check non-empty after trim
                if self.#field_ident.to_field_value().trim().is_empty() {
                    return Err(borang::ValidationError::with_kind(
                        borang::ErrorKind::Required {
                            field: #field_name.to_string(),
                        }
                    ));
                }
            }
        }

        Validator::Email => {
            quote! {
                // Email validation using regex
                let email_value = self.#field_ident.to_field_value();
                if !email_value.is_empty() {
                    // Simple email regex pattern
                    let email_pattern = regex::Regex::new(
                        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
                    ).unwrap();
                    if !email_pattern.is_match(&email_value) {
                        return Err(borang::ValidationError::with_kind(
                            borang::ErrorKind::InvalidEmail {
                                field: #field_name.to_string(),
                            }
                        ));
                    }
                }
            }
        }

        Validator::Length { min, max } => {
            let min_opt = min
                .as_ref()
                .map(|v| quote! { Some(#v) })
                .unwrap_or(quote! { None });
            let max_opt = max
                .as_ref()
                .map(|v| quote! { Some(#v) })
                .unwrap_or(quote! { None });

            let mut checks = Vec::new();

            if let Some(min_val) = min {
                checks.push(quote! {
                    if value.len() < #min_val {
                        return Err(borang::ValidationError::with_kind(
                            borang::ErrorKind::InvalidLength {
                                field: #field_name.to_string(),
                                min: #min_opt,
                                max: #max_opt,
                            }
                        ));
                    }
                });
            }

            if let Some(max_val) = max {
                checks.push(quote! {
                    if value.len() > #max_val {
                        return Err(borang::ValidationError::with_kind(
                            borang::ErrorKind::InvalidLength {
                                field: #field_name.to_string(),
                                min: #min_opt,
                                max: #max_opt,
                            }
                        ));
                    }
                });
            }

            quote! {
                // Length validation
                let value = self.#field_ident.to_field_value();
                #(#checks)*
            }
        }

        Validator::Range { min, max } => {
            let min_opt = min
                .as_ref()
                .map(|v| quote! { Some(#v) })
                .unwrap_or(quote! { None });
            let max_opt = max
                .as_ref()
                .map(|v| quote! { Some(#v) })
                .unwrap_or(quote! { None });

            let mut checks = Vec::new();

            if let Some(min_val) = min {
                checks.push(quote! {
                    if value < #min_val {
                        return Err(borang::ValidationError::with_kind(
                            borang::ErrorKind::InvalidRange {
                                field: #field_name.to_string(),
                                min: #min_opt,
                                max: #max_opt,
                            }
                        ));
                    }
                });
            }

            if let Some(max_val) = max {
                checks.push(quote! {
                    if value > #max_val {
                        return Err(borang::ValidationError::with_kind(
                            borang::ErrorKind::InvalidRange {
                                field: #field_name.to_string(),
                                min: #min_opt,
                                max: #max_opt,
                            }
                        ));
                    }
                });
            }

            quote! {
                // Range validation - convert to i64 for comparison
                let value = self.#field_ident as i64;
                #(#checks)*
            }
        }

        Validator::Custom { method_name } => {
            let method_ident = syn::Ident::new(method_name, proc_macro2::Span::call_site());
            quote! {
                // Custom validation
                self.#method_ident()?;
            }
        }
    }
}

/// Generate the validate_field match arm for a single field
fn generate_validate_field_arm(field_validation: &FieldValidation) -> proc_macro2::TokenStream {
    let field_name = &field_validation.field_name;
    let field_type = &field_validation.field_type;

    let validator_code: Vec<_> = field_validation
        .validators
        .iter()
        .map(|v| generate_validator_code(field_name, field_type, v))
        .collect();

    quote! {
        #field_name => {
            #(#validator_code)*
            Ok(())
        }
    }
}

/// Derive macro for generating form validation implementations.
///
/// This macro generates the `FormValidation` trait implementation for a struct,
/// parsing `#[validator(...)]` attributes on fields to generate validation logic.
///
/// # Example
///
/// ```ignore
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
#[proc_macro_derive(FormValidation, attributes(validator))]
pub fn derive_form_validation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Extract field validations
    let field_validations = match extract_field_validations(&input.data) {
        Ok(validations) => validations,
        Err(e) => return TokenStream::from(e.to_compile_error()),
    };

    // Generate validate_field match arms
    let validate_field_arms: Vec<_> = field_validations
        .iter()
        .map(generate_validate_field_arm)
        .collect();

    // Generate field names list
    let field_names: Vec<_> = field_validations.iter().map(|fv| &fv.field_name).collect();

    // Generate validate_all implementation
    let validate_all_calls: Vec<_> = field_validations
        .iter()
        .map(|fv| {
            let field_name = &fv.field_name;
            quote! {
                if let Err(e) = self.validate_field(#field_name) {
                    errors.insert(#field_name.to_string(), e);
                }
            }
        })
        .collect();

    // Generate sync_from_strings implementation
    let sync_from_strings_code: Vec<_> = field_validations
        .iter()
        .map(|fv| {
            let field_name = &fv.field_name;
            let field_ident = syn::Ident::new(field_name, proc_macro2::Span::call_site());
            let field_type = &fv.field_type;

            quote! {
                if let Some(field) = fields.get(#field_name) {
                    let value = field.value.get_untracked();
                    match <#field_type as borang::FromFieldValue>::from_field_value(#field_name, &value) {
                        Ok(parsed) => self.#field_ident = parsed,
                        Err(e) => { errors.insert(#field_name.to_string(), e); }
                    }
                }
            }
        })
        .collect();

    // Generate to_strings implementation
    let to_strings_code: Vec<_> = field_validations
        .iter()
        .map(|fv| {
            let field_name = &fv.field_name;
            let field_ident = syn::Ident::new(field_name, proc_macro2::Span::call_site());

            quote! {
                map.insert(#field_name.to_string(), borang::FromFieldValue::to_field_value(&self.#field_ident));
            }
        })
        .collect();

    let expanded = quote! {
        impl borang::FormValidation for #name {
            fn validate_all(&self) -> std::collections::HashMap<String, borang::ValidationError> {
                use borang::FromFieldValue;

                let mut errors = std::collections::HashMap::new();

                #(#validate_all_calls)*

                errors
            }

            fn validate_field(&self, field_name: &str) -> borang::ValidationResult {
                use borang::FromFieldValue;

                match field_name {
                    #(#validate_field_arms,)*
                    _ => Ok(()),
                }
            }

            fn field_names() -> Vec<&'static str> {
                vec![#(#field_names),*]
            }

            fn sync_from_strings(
                &mut self,
                fields: &std::collections::HashMap<String, borang::FieldSignal>
            ) -> std::collections::HashMap<String, borang::ValidationError> {
                use leptos::prelude::*;

                let mut errors = std::collections::HashMap::new();

                #(#sync_from_strings_code)*

                errors
            }

            fn to_strings(&self) -> std::collections::HashMap<String, String> {
                use borang::FromFieldValue;

                let mut map = std::collections::HashMap::new();

                #(#to_strings_code)*

                map
            }
        }
    };

    TokenStream::from(expanded)
}
