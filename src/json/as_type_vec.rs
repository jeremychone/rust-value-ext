use crate::json::as_type_str::AsType;
use crate::JsonValueExtError;
use serde_json::Value;

impl<'a> AsType<'a> for &'a Vec<Value> {
	fn from_value(value: &'a Value) -> Result<Self, JsonValueExtError> {
		value.as_array().ok_or(JsonValueExtError::ValueNotOfType("Vec<Value>"))
	}
}

impl<'a> AsType<'a> for Vec<&'a str> {
	fn from_value(value: &'a Value) -> Result<Self, JsonValueExtError> {
		let arr = value.as_array().ok_or(JsonValueExtError::ValueNotOfType("Vec<&str>"))?;
		let mut result = Vec::with_capacity(arr.len());
		for item in arr {
			if let Some(s) = item.as_str() {
				result.push(s);
			} else {
				return Err(JsonValueExtError::ValueNotOfType("Vec<&str>"));
			}
		}
		Ok(result)
	}
}
