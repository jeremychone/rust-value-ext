use crate::json::as_type_str::AsType;
use crate::JsonValueExtError;
use serde_json::Value;

impl AsType<'_> for f64 {
	fn from_value(value: &Value) -> Result<Self, JsonValueExtError> {
		value.as_f64().ok_or(JsonValueExtError::ValueNotOfType("f64"))
	}
}

impl AsType<'_> for Option<f64> {
	fn from_value(value: &Value) -> Result<Self, JsonValueExtError> {
		Ok(value.as_f64())
	}
}

impl AsType<'_> for i64 {
	fn from_value(value: &Value) -> Result<Self, JsonValueExtError> {
		value.as_i64().ok_or(JsonValueExtError::ValueNotOfType("i64"))
	}
}

impl AsType<'_> for Option<i64> {
	fn from_value(value: &Value) -> Result<Self, JsonValueExtError> {
		Ok(value.as_i64())
	}
}

impl AsType<'_> for i32 {
	fn from_value(value: &Value) -> Result<Self, JsonValueExtError> {
		value
			.as_i64()
			.and_then(|v| i32::try_from(v).ok())
			.ok_or(JsonValueExtError::ValueNotOfType("i32"))
	}
}

impl AsType<'_> for Option<i32> {
	fn from_value(value: &Value) -> Result<Self, JsonValueExtError> {
		Ok(value.as_i64().and_then(|v| i32::try_from(v).ok()))
	}
}

impl AsType<'_> for u32 {
	fn from_value(value: &Value) -> Result<Self, JsonValueExtError> {
		value
			.as_u64()
			.and_then(|v| u32::try_from(v).ok())
			.ok_or(JsonValueExtError::ValueNotOfType("u32"))
	}
}

impl AsType<'_> for Option<u32> {
	fn from_value(value: &Value) -> Result<Self, JsonValueExtError> {
		Ok(value.as_u64().and_then(|v| u32::try_from(v).ok()))
	}
}

impl AsType<'_> for bool {
	fn from_value(value: &Value) -> Result<Self, JsonValueExtError> {
		value.as_bool().ok_or(JsonValueExtError::ValueNotOfType("bool"))
	}
}

impl AsType<'_> for Option<bool> {
	fn from_value(value: &Value) -> Result<Self, JsonValueExtError> {
		Ok(value.as_bool())
	}
}
