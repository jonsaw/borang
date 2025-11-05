use crate::{ValidationError, ValidationResult, ValidationRule};

pub struct WithMessage<
    T: Send + Sync,
    R: ValidationRule<T>,
    F: Fn(ValidationError) -> String + Clone + Send + Sync + 'static,
> {
    rule: R,
    message_fn: F,
    _phantom: std::marker::PhantomData<T>,
}

impl<
    T: Send + Sync,
    R: ValidationRule<T>,
    F: Fn(ValidationError) -> String + Clone + Send + Sync + 'static,
> WithMessage<T, R, F>
{
    pub fn new(rule: R, message_fn: F) -> Self {
        Self {
            rule,
            message_fn,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<
    T: Send + Sync,
    R: ValidationRule<T>,
    F: Fn(ValidationError) -> String + Clone + Send + Sync + 'static,
> ValidationRule<T> for WithMessage<T, R, F>
{
    fn validate(&self, field_name: &str, value: &T) -> ValidationResult {
        let message_fn = self.message_fn.clone();
        self.rule.validate(field_name, value).map_err(|err| {
            ValidationError::new(field_name.to_string(), move || message_fn(err.clone()))
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
            Err(ValidationError::new(field_name.to_string(), || {
                "is required".to_string()
            }))
        } else {
            Ok(())
        }
    }
}

pub struct Email;

impl ValidationRule<String> for Email {
    fn validate(&self, field_name: &str, value: &String) -> ValidationResult {
        let regex = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

        if !regex.is_match(value) {
            Err(ValidationError::new(field_name.to_string(), || {
                "is not a valid email".to_string()
            }))
        } else {
            Ok(())
        }
    }
}

pub struct Length {
    min: Option<usize>,
    max: Option<usize>,
}

impl Length {
    pub fn min(min: usize) -> Self {
        Self {
            min: Some(min),
            max: None,
        }
    }

    pub fn max(mut self, max: usize) -> Self {
        self.max = Some(max);
        self
    }
}

impl ValidationRule<String> for Length {
    fn validate(&self, field_name: &str, value: &String) -> ValidationResult {
        let len = value.chars().count();

        if let Some(min) = self.min {
            if len < min {
                return Err(ValidationError::new(field_name.to_string(), move || {
                    format!("must be at least {} characters", min)
                }));
            }
        }

        if let Some(max) = self.max {
            if len > max {
                return Err(ValidationError::new(field_name.to_string(), move || {
                    format!("must be at most {} characters", max)
                }));
            }
        }

        Ok(())
    }
}
