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
/// - **`x_get`**: Returns a value of a specified type `T` from a JSON object using either a direct name or a pointer path.
/// - **`x_take`**: Takes a value from a JSON object using a specified name or pointer path, replacing it with `Null`.
/// - **`x_insert`**: Inserts a value of type `T` into a JSON object at the specified name or pointer path, creating any missing objects along the way.
/// - **`x_walk`**: Traverses all properties within the JSON value tree, applying a user-provided callback function on each property.
/// - **`x_pretty`**: Returns a pretty-printed string representation of the JSON value.
///
/// # Usage
///
/// This trait is intended to be used with `serde_json::Value` objects. It is particularly
/// useful when you need to manipulate JSON structures dynamically or when the structure
/// of the JSON is not known at compile time.
///
/// ```rust
/// use serde_json::{Value, Map};
/// use serde::de::DeserializeOwned;
/// use serde::Serialize;
/// use your_crate::JsonValueExt;
///
/// fn example_usage(json: &mut Value) -> Result<(), Box<dyn std::error::Error>> {
///     // Get a value from JSON
///     let name: String = json.x_get("/name")?;
///
///     // Take a value from JSON, replacing it with `Null`
///     let age: u32 = json.x_take("age")?;
///
///     // Insert a new value into JSON
///     json.x_insert("city", "New York")?;
///
///     // Walk through the JSON properties
///     json.x_walk(|parent_map, property_name| {
///         println!("Property: {}", property_name);
///         true // Continue traversal
///     });
///
///     // Get a pretty-printed JSON string
///     let pretty_json = json.x_pretty()?;
///     println!("{}", pretty_json);
///
///     Ok(())
/// }
/// ```
///
/// This trait enhances the `serde_json::Value` API by adding more type-safe and convenient
/// methods for manipulating JSON data in Rust.
pub trait JsonValueExt {
	fn x_new_object() -> Value;

	/// Returns an owned type `T` for a given name or pointer path.
	/// - `name_or_pointer`: Can be a direct name or a pointer path (if it starts with '/').
	fn x_get<T: DeserializeOwned>(&self, name_or_pointer: &str) -> Result<T>;

	/// Returns a reference of type `T` (or value for copy type) for a given name or pointer path.
	/// Use this one over `x_get` to avoid string allocation, and get only the &str
	/// - `name_or_pointer`: Can be a direct name or a pointer path (if it starts with '/').
	fn x_get_as<'a, T: AsType<'a>>(&'a self, name_or_pointer: &str) -> Result<T>;

	/// Takes the value at the specified name or pointer path and replaces it with `Null`.
	/// - `name_or_pointer`: Can be a direct name or a pointer path (if it starts with '/').
	fn x_take<T: DeserializeOwned>(&mut self, name_or_pointer: &str) -> Result<T>;

	/// Inserts a new value of type `T` at the specified name or pointer path.
	/// This method creates missing `Value::Object` entries as needed.
	/// - `name_or_pointer`: Can be a direct name or a pointer path (if it starts with '/').
	fn x_insert<T: Serialize>(&mut self, name_or_pointer: &str, value: T) -> Result<()>;

	/// Walks through all properties in the JSON value tree and calls the callback function on each.
	/// - The callback signature is `(parent_map, property_name) -> bool`.
	///   - Returns `false` to stop the traversal; returns `true` to continue.
	///
	/// Returns:
	/// - `true` if the traversal completes without stopping early.
	/// - `false` if the traversal is stopped early because the callback returned `false`.
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

	fn x_get<T: DeserializeOwned>(&self, name_or_pointer: &str) -> Result<T> {
		let value = if name_or_pointer.starts_with('/') {
			self.pointer(name_or_pointer)
				.ok_or_else(|| JsonValueExtError::PropertyNotFound(name_or_pointer.to_string()))?
		} else {
			self.get(name_or_pointer)
				.ok_or_else(|| JsonValueExtError::PropertyNotFound(name_or_pointer.to_string()))?
		};

		let value: T = serde_json::from_value(value.clone())?;
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

		T::from_value(value)
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

	fn x_insert<T: Serialize>(&mut self, name_or_pointer: &str, value: T) -> Result<()> {
		let new_value = serde_json::to_value(value)?;

		if !name_or_pointer.starts_with('/') {
			match self {
				Value::Object(map) => {
					map.insert(name_or_pointer.to_string(), new_value);
					Ok(())
				}
				_ => Err(JsonValueExtError::custom("Value is not an Object, cannot x_insert")),
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
				// If current value is an array, add its elements to the queue
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

	// -- AsType errors
	ValueNotType(&'static str),

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
