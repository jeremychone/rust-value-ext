use crate::AsType;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, Map, Value};
use std::collections::VecDeque;

/// Extension trait for working with JSON values in a more convenient way.
///
/// `JsonValueExt` offers convenient methods for interacting with `serde_json::Value` objects,
/// simplifying tasks like getting, taking, inserting, traversing, and pretty-printing JSON data
/// while ensuring type safety with Serde's serialization and deserialization.
///
/// # Provided Methods
///
/// - **`x_get`**: Returns an owned value of a specified type `T` from a JSON object using either a direct name or a pointer path.
/// - **`x_get_as`**: Returns a reference of a specified type `T` from a JSON object using either a direct name or a pointer path.
/// - **`x_get_str`**: Returns a `&str` from a JSON object using either a direct name or a pointer path.
/// - **`x_get_i64`**: Returns an `i64` from a JSON object using either a direct name or a pointer path.
/// - **`x_get_f64`**: Returns an `f64` from a JSON object using either a direct name or a pointer path.
/// - **`x_get_bool`**: Returns a `bool` from a JSON object using either a direct name or a pointer path.
/// - **`x_take`**: Takes a value from a JSON object using a specified name or pointer path, replacing it with `Null`.
/// - **`x_remove`**: Removes the value at the specified name or pointer path from the JSON object and returns it,
///   leaving no placeholder in the object (unlike `x_take`).
/// - **`x_insert`**: Inserts a new value of type `T` into a JSON object at the specified name or pointer path,
///   creating any missing objects along the way.
/// - **`x_walk`**: Traverses all properties in the JSON value tree and calls the callback function on each.
/// - **`x_pretty`**: Returns a pretty-printed string representation of the JSON value.
pub trait JsonValueExt {
	fn x_new_object() -> Value;

	fn x_contains<T: DeserializeOwned>(&self, name_or_pointer: &str) -> bool;

	/// Returns an owned type `T` for a given name or pointer path.
	/// Note: This will create a clone of the matched Value.
	/// - `name_or_pointer`: Can be a direct name or a pointer path (if it starts with '/').
	fn x_get<T: DeserializeOwned>(&self, name_or_pointer: &str) -> Result<T>;

	/// Returns a reference of type `T` (or a copy for copy types) for a given name or pointer path.
	/// Use this one over `x_get` to avoid string allocation.
	/// - `name_or_pointer`: Can be a direct name or a pointer path (if it starts with '/').
	fn x_get_as<'a, T: AsType<'a>>(&'a self, name_or_pointer: &str) -> Result<T>;

	/// Returns a &str if present (shortcut for `x_get_as::<&str>(...)`)
	fn x_get_str(&self, name_or_pointer: &str) -> Result<&str> {
		self.x_get_as(name_or_pointer)
	}

	/// Returns an i64 if present (shortcut for `x_get_as::<i64>(...)`)
	fn x_get_i64(&self, name_or_pointer: &str) -> Result<i64> {
		self.x_get_as(name_or_pointer)
	}

	/// Returns an f64 if present (shortcut for `x_get_as::<f64>(...)`)
	fn x_get_f64(&self, name_or_pointer: &str) -> Result<f64> {
		self.x_get_as(name_or_pointer)
	}

	/// Returns a bool if present (shortcut for `x_get_as::<bool>(...)`)
	fn x_get_bool(&self, name_or_pointer: &str) -> Result<bool> {
		self.x_get_as(name_or_pointer)
	}

	/// Takes the value at the specified name or pointer path and replaces it with `Null`.
	/// - `name_or_pointer`: Can be a direct name or a pointer path (if it starts with '/').
	fn x_take<T: DeserializeOwned>(&mut self, name_or_pointer: &str) -> Result<T>;

	/// Removes the value at the specified name or pointer path from the JSON object
	/// and returns it without leaving a placeholder, unlike `x_take`.
	fn x_remove<T: DeserializeOwned>(&mut self, name_or_pointer: &str) -> Result<T>;

	/// Inserts a new value of type `T` at the specified name or pointer path.
	/// This method creates missing `Value::Object` entries as needed.
	/// - `name_or_pointer`: Can be a direct name or a pointer path (if it starts with '/').
	fn x_insert<T: Serialize>(&mut self, name_or_pointer: &str, value: T) -> Result<()>;

	/// Merges another JSON object into this one (shallow merge).
	/// - If `self` is not an Object, returns error.
	/// - If `other` is Null, does nothing.
	/// - If `other` is not an Object, returns error.
	fn x_merge(&mut self, other: Value) -> Result<()>;

	/// Walks through all properties in the JSON value tree and calls the callback function on each.
	/// - The callback signature is `(parent_map, property_name) -> bool`.
	///   - Returns `false` to stop the traversal; returns `true` to continue.
	///
	/// Returns:
	/// - `true` if the traversal completes without stopping early.
	/// - `false` if the traversal was stopped early because the callback returned `false`.
	fn x_walk<F>(&mut self, callback: F) -> bool
	where
		F: FnMut(&mut Map<String, Value>, &str) -> bool;

	/// Returns a pretty-printed string representation of the JSON value.
	fn x_pretty(&self) -> Result<String>;
}

impl JsonValueExt for Value {
	fn x_new_object() -> Value {
		Value::Object(Map::new())
	}

	fn x_contains<T: DeserializeOwned>(&self, name_or_pointer: &str) -> bool {
		if name_or_pointer.starts_with('/') {
			self.pointer(name_or_pointer).is_some()
		} else {
			self.get(name_or_pointer).is_some()
		}
	}

	fn x_get<T: DeserializeOwned>(&self, name_or_pointer: &str) -> Result<T> {
		let value = if name_or_pointer.starts_with('/') {
			self.pointer(name_or_pointer)
				.ok_or_else(|| JsonValueExtError::PropertyNotFound(name_or_pointer.to_string()))?
		} else {
			self.get(name_or_pointer)
				.ok_or_else(|| JsonValueExtError::PropertyNotFound(name_or_pointer.to_string()))?
		};

		let value: T =
			serde_json::from_value(value.clone())
				.map_err(JsonValueExtError::from)
				.map_err(|err| match err {
					JsonValueExtError::ValueNotOfType(not_of_type) => JsonValueExtError::PropertyValueNotOfType {
						name: name_or_pointer.to_string(),
						not_of_type,
					},
					other => other,
				})?;

		Ok(value)
	}

	fn x_get_as<'a, T: AsType<'a>>(&'a self, name_or_pointer: &str) -> Result<T> {
		let value = if name_or_pointer.starts_with('/') {
			self.pointer(name_or_pointer)
				.ok_or_else(|| JsonValueExtError::PropertyNotFound(name_or_pointer.to_string()))?
		} else {
			self.get(name_or_pointer)
				.ok_or_else(|| JsonValueExtError::PropertyNotFound(name_or_pointer.to_string()))?
		};

		T::from_value(value).map_err(|err| match err {
			JsonValueExtError::ValueNotOfType(not_of_type) => JsonValueExtError::PropertyValueNotOfType {
				name: name_or_pointer.to_string(),
				not_of_type,
			},
			other => other,
		})
	}

	fn x_take<T: DeserializeOwned>(&mut self, name_or_pointer: &str) -> Result<T> {
		let value = if name_or_pointer.starts_with('/') {
			self.pointer_mut(name_or_pointer)
				.map(Value::take)
				.ok_or_else(|| JsonValueExtError::PropertyNotFound(name_or_pointer.to_string()))?
		} else {
			self.get_mut(name_or_pointer)
				.map(Value::take)
				.ok_or_else(|| JsonValueExtError::PropertyNotFound(name_or_pointer.to_string()))?
		};

		let value: T = serde_json::from_value(value)?;
		Ok(value)
	}

	fn x_remove<T: DeserializeOwned>(&mut self, name_or_pointer: &str) -> Result<T> {
		if !name_or_pointer.starts_with('/') {
			match self {
				Value::Object(map) => {
					let removed = map
						.remove(name_or_pointer)
						.ok_or_else(|| JsonValueExtError::PropertyNotFound(name_or_pointer.to_string()))?;
					let value: T = serde_json::from_value(removed)?;
					Ok(value)
				}
				_ => Err(JsonValueExtError::custom("Value is not an Object; cannot x_remove")),
			}
		} else {
			let parts: Vec<&str> = name_or_pointer.split('/').skip(1).collect();
			if parts.is_empty() {
				return Err(JsonValueExtError::custom("Invalid path"));
			}
			let mut current = self;
			for &part in &parts[..parts.len() - 1] {
				match current {
					Value::Object(map) => {
						current = map
							.get_mut(part)
							.ok_or_else(|| JsonValueExtError::PropertyNotFound(name_or_pointer.to_string()))?;
					}
					Value::Array(arr) => {
						let index: usize = part
							.parse()
							.map_err(|_| JsonValueExtError::custom("Invalid array index in pointer"))?;
						if index < arr.len() {
							current = &mut arr[index];
						} else {
							return Err(JsonValueExtError::PropertyNotFound(name_or_pointer.to_string()));
						}
					}
					_ => return Err(JsonValueExtError::custom("Path does not point to an Object or Array")),
				}
			}
			let last_part = parts
				.last()
				.ok_or_else(|| JsonValueExtError::custom("Last element not found"))?;
			match current {
				Value::Object(map) => {
					let removed = map
						.remove(*last_part)
						.ok_or_else(|| JsonValueExtError::PropertyNotFound(name_or_pointer.to_string()))?;
					let value: T = serde_json::from_value(removed)?;
					Ok(value)
				}
				Value::Array(arr) => {
					let index: usize = last_part
						.parse()
						.map_err(|_| JsonValueExtError::custom("Invalid array index in pointer"))?;
					if index < arr.len() {
						let removed = arr.remove(index);
						let value: T = serde_json::from_value(removed)?;
						Ok(value)
					} else {
						Err(JsonValueExtError::PropertyNotFound(name_or_pointer.to_string()))
					}
				}
				_ => Err(JsonValueExtError::custom("Path does not point to an Object or Array")),
			}
		}
	}

	fn x_insert<T: Serialize>(&mut self, name_or_pointer: &str, value: T) -> Result<()> {
		let new_value = serde_json::to_value(value)?;

		if !name_or_pointer.starts_with('/') {
			match self {
				Value::Object(map) => {
					map.insert(name_or_pointer.to_string(), new_value);
					Ok(())
				}
				_ => Err(JsonValueExtError::custom("Value is not an Object; cannot x_insert")),
			}
		} else {
			let parts: Vec<&str> = name_or_pointer.split('/').skip(1).collect();
			let mut current = self;

			// -- Add the eventual missing parents
			for &part in &parts[..parts.len() - 1] {
				match current {
					Value::Object(map) => {
						current = map.entry(part).or_insert_with(|| json!({}));
					}
					_ => return Err(JsonValueExtError::custom("Path does not point to an Object")),
				}
			}

			// -- Set the value at the last element
			if let Some(&last_part) = parts.last() {
				match current {
					Value::Object(map) => {
						map.insert(last_part.to_string(), new_value);
						Ok(())
					}
					_ => Err(JsonValueExtError::custom("Path does not point to an Object")),
				}
			} else {
				Err(JsonValueExtError::custom("Invalid path"))
			}
		}
	}

	fn x_merge(&mut self, other: Value) -> Result<()> {
		if other.is_null() {
			return Ok(());
		}

		let other_map = match other {
			Value::Object(map) => map,
			_ => return Err(JsonValueExtError::custom("Other value is not an Object; cannot x_merge")),
		};

		match self {
			Value::Object(map) => {
				map.extend(other_map);
				Ok(())
			}
			_ => Err(JsonValueExtError::custom("Value is not an Object; cannot x_merge")),
		}
	}

	fn x_pretty(&self) -> Result<String> {
		let content = serde_json::to_string_pretty(self)?;
		Ok(content)
	}

	/// Walks through all properties of a JSON value tree and calls the callback function on each property.
	///
	/// - The callback signature is `(parent_map, property_name) -> bool`.
	///   - Return `false` from the callback to stop the traversal; return `true` to continue.
	///
	/// Returns:
	/// - `true` if the traversal completed to the end without being stopped early.
	/// - `false` if the traversal was stopped early because the callback returned `false`.
	fn x_walk<F>(&mut self, mut callback: F) -> bool
	where
		F: FnMut(&mut Map<String, Value>, &str) -> bool,
	{
		let mut queue = VecDeque::new();
		queue.push_back(self);

		while let Some(current) = queue.pop_front() {
			if let Value::Object(map) = current {
				// Call the callback for each property name in the current map
				for key in map.keys().cloned().collect::<Vec<_>>() {
					let res = callback(map, &key);
					if !res {
						return false;
					}
				}

				// Add all nested objects and arrays to the queue for further processing
				for value in map.values_mut() {
					if value.is_object() || value.is_array() {
						queue.push_back(value);
					}
				}
			} else if let Value::Array(arr) = current {
				// If the current value is an array, add its elements to the queue
				for value in arr.iter_mut() {
					if value.is_object() || value.is_array() {
						queue.push_back(value);
					}
				}
			}
		}
		true
	}
}

// region:    --- Error
type Result<T> = core::result::Result<T, JsonValueExtError>;

#[derive(Debug, derive_more::From)]
pub enum JsonValueExtError {
	Custom(String),

	PropertyNotFound(String),

	PropertyValueNotOfType {
		name: String,
		not_of_type: &'static str,
	},

	// -- AsType errors
	ValueNotOfType(&'static str),

	#[from]
	SerdeJson(serde_json::Error),
}

impl JsonValueExtError {
	pub(crate) fn custom(val: impl std::fmt::Display) -> Self {
		Self::Custom(val.to_string())
	}
}

// region:    --- Error Boilerplate

impl core::fmt::Display for JsonValueExtError {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for JsonValueExtError {}

// endregion: --- Error Boilerplate

// endregion: --- Error
