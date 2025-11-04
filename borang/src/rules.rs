use crate::{ValidationError, ValidationResult, ValidationRule};

pub struct WithMessage<T: Send + Sync, R: ValidationRule<T>> {
    rule: R,
    message: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Send + Sync, R: ValidationRule<T>> WithMessage<T, R> {
    pub fn new(rule: R, message: impl Into<String>) -> Self {
        Self {
            rule,
            message: message.into(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Send + Sync, R: ValidationRule<T>> ValidationRule<T> for WithMessage<T, R> {
    fn validate(&self, field_name: &str, value: &T) -> ValidationResult {
        self.rule.validate(field_name, value).map_err(|mut err| {
            err.message = self.message.clone();
            err
        })
    }
}

pub struct Rules<T> {
    rules: Vec<Box<dyn ValidationRule<T>>>,
}

impl<T> Rules<T> {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add<R: ValidationRule<T> + 'static>(mut self, rule: R) -> Self {
        self.rules.push(Box::new(rule));
        self
    }
}

impl<T> ValidationRule<T> for Rules<T> {
    fn validate(&self, field_name: &str, value: &T) -> ValidationResult {
        for rule in &self.rules {
            let result = rule.validate(field_name, value);
            match &result {
                Ok(()) => continue,
                Err(_err) => return result,
            }
        }
        Ok(())
    }
}

pub struct Required;

impl ValidationRule<String> for Required {
    fn validate(&self, field_name: &str, value: &String) -> ValidationResult {
        if value.trim().is_empty() {
            Err(ValidationError {
                field: field_name.to_string(),
                message: "is required".to_string(),
            })
        } else {
            Ok(())
        }
    }
}
