use crate::JsonValueExtError;
use serde_json::Value;

pub trait AsType<'a>: Sized {
	fn from_value(value: &'a Value) -> Result<Self, JsonValueExtError>;
}

impl<'a> AsType<'a> for &'a str {
	fn from_value(value: &'a Value) -> Result<Self, JsonValueExtError> {
		value.as_str().ok_or(JsonValueExtError::ValueNotType("str"))
	}
}

impl AsType<'_> for f64 {
	fn from_value(value: &Value) -> Result<Self, JsonValueExtError> {
		value.as_f64().ok_or(JsonValueExtError::ValueNotType("f64"))
	}
}

impl AsType<'_> for i64 {
	fn from_value(value: &Value) -> Result<Self, JsonValueExtError> {
		value.as_i64().ok_or(JsonValueExtError::ValueNotType("i64"))
	}
}

impl AsType<'_> for i32 {
	fn from_value(value: &Value) -> Result<Self, JsonValueExtError> {
		value
			.as_i64()
			.and_then(|v| i32::try_from(v).ok())
			.ok_or(JsonValueExtError::ValueNotType("i32"))
	}
}

impl AsType<'_> for u32 {
	fn from_value(value: &Value) -> Result<Self, JsonValueExtError> {
		value
			.as_u64()
			.and_then(|v| u32::try_from(v).ok())
			.ok_or(JsonValueExtError::ValueNotType("u32"))
	}
}

impl AsType<'_> for bool {
	fn from_value(value: &Value) -> Result<Self, JsonValueExtError> {
		value.as_bool().ok_or(JsonValueExtError::ValueNotType("bool"))
	}
}
