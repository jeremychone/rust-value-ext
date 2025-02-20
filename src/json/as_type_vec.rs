use crate::JsonValueExtError;
use crate::json::as_type_str::AsType;
use serde_json::Value;

impl<'a> AsType<'a> for &'a Vec<Value> {
	fn from_value(value: &'a Value) -> Result<Self, JsonValueExtError> {
		value.as_array().ok_or(JsonValueExtError::ValueNotOfType("Vec<Value>"))
	}
}
