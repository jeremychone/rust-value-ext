use crate::JsonValueExtError;
use crate::json::as_type_str::AsType;
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

impl AsType<'_> for Vec<i64> {
	fn from_value(value: &Value) -> Result<Self, JsonValueExtError> {
		let arr = value.as_array().ok_or(JsonValueExtError::ValueNotOfType("Vec<i64>"))?;
		let mut result = Vec::with_capacity(arr.len());
		for item in arr {
			if let Some(v) = item.as_i64() {
				result.push(v);
			} else {
				return Err(JsonValueExtError::ValueNotOfType("Vec<i64>"));
			}
		}
		Ok(result)
	}
}

impl AsType<'_> for Vec<f64> {
	fn from_value(value: &Value) -> Result<Self, JsonValueExtError> {
		let arr = value.as_array().ok_or(JsonValueExtError::ValueNotOfType("Vec<f64>"))?;
		let mut result = Vec::with_capacity(arr.len());
		for item in arr {
			if let Some(v) = item.as_f64() {
				result.push(v);
			} else {
				return Err(JsonValueExtError::ValueNotOfType("Vec<f64>"));
			}
		}
		Ok(result)
	}
}

impl AsType<'_> for Vec<bool> {
	fn from_value(value: &Value) -> Result<Self, JsonValueExtError> {
		let arr = value.as_array().ok_or(JsonValueExtError::ValueNotOfType("Vec<bool>"))?;
		let mut result = Vec::with_capacity(arr.len());
		for item in arr {
			if let Some(v) = item.as_bool() {
				result.push(v);
			} else {
				return Err(JsonValueExtError::ValueNotOfType("Vec<bool>"));
			}
		}
		Ok(result)
	}
}
