use serde_json::Value;
use std::borrow::Cow;

#[allow(dead_code)]
pub fn format_validation_errors(errors: &validator::ValidationErrors) -> Value {
    let formatted_errors = errors
        .field_errors()
        .iter()
        .map(|(field, errors)| {
            let messages: Vec<String> = errors
                .iter()
                .map(|error| {
                    error
                        .message
                        .clone()
                        .unwrap_or_else(|| Cow::from("Invalid value"))
                })
                .map(|cow| cow.into_owned())
                .collect();

            (
                field.to_string(),
                Value::Array(messages.into_iter().map(Value::String).collect()),
            )
        })
        .collect::<serde_json::Map<String, Value>>();

    Value::Object(formatted_errors)
}
