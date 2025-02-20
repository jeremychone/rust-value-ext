use crate::JsonValueExtError;
use serde_json::Value;

pub trait AsType<'a>: Sized {
	fn from_value(value: &'a Value) -> Result<Self, JsonValueExtError>;
}

impl<'a> AsType<'a> for &'a str {
	fn from_value(value: &'a Value) -> Result<Self, JsonValueExtError> {
		value.as_str().ok_or(JsonValueExtError::ValueNotOfType("str"))
	}
}

impl<'a> AsType<'a> for Option<&'a str> {
	fn from_value(value: &'a Value) -> Result<Self, JsonValueExtError> {
		Ok(value.as_str())
	}
}
