# value-ext API Reference (v0.1.3-WIP)

Extension trait for `serde_json::Value` providing convenient, type-safe accessors and mutations.

## JsonValueExt Trait

Implemented for `serde_json::Value`. Methods taking `name_or_pointer` accept a direct property name or a JSON Pointer (starting with `/`).

### Creation & Checks
- `x_new_object() -> Value`: Returns `Value::Object(Map::new())`.
- `x_contains<T>(&self, name_or_pointer: &str) -> bool`: Checks if path exists.

### Getters (Owned)
- `x_get<T: DeserializeOwned>(&self, name_or_pointer: &str) -> Result<T>`: Returns owned value (clones internal value).

### Getters (Ref / AsType)
- `x_get_as<'a, T: AsType<'a>>(&'a self, name_or_pointer: &str) -> Result<T>`: Specialized conversion (often zero-copy).
- `x_get_str(&self, name_or_pointer: &str) -> Result<&str>`: Shortcut for `x_get_as::<&str>`.
- `x_get_i64(&self, name_or_pointer: &str) -> Result<i64>`: Shortcut for `x_get_as::<i64>`.
- `x_get_f64(&self, name_or_pointer: &str) -> Result<f64>`: Shortcut for `x_get_as::<f64>`.
- `x_get_bool(&self, name_or_pointer: &str) -> Result<bool>`: Shortcut for `x_get_as::<bool>`.

### Mutations
- `x_take<T: DeserializeOwned>(&mut self, name_or_pointer: &str) -> Result<T>`: Replaces value at path with `Null` and returns it.
- `x_remove<T: DeserializeOwned>(&mut self, name_or_pointer: &str) -> Result<T>`: Removes property from map/array and returns it.
- `x_insert<T: Serialize>(&mut self, name_or_pointer: &str, value: T) -> Result<()>`: Inserts value at path, creating missing objects.
- `x_merge(&mut self, other: Value) -> Result<()>`: Shallow merge of two JSON objects.

### Utilities
- `x_walk<F>(&mut self, callback: F) -> bool`: BFS traversal. Callback: `(parent_map, key) -> bool`. Return `false` to stop.
- `x_pretty(&self) -> Result<String>`: Pretty-printed JSON string.

## Supported AsType<'a> Implementations

Used with `x_get_as`:
- `&str`, `Option<&str>`
- `f64`, `Option<f64>`
- `i64`, `Option<i64>`
- `i32`, `Option<i32>`
- `u32`, `Option<u32>`
- `bool`, `Option<bool>`
- `&Vec<Value>`: References the internal array.
- `Vec<&str>`: Collection of references to internal strings.

## Errors (JsonValueExtError)

- `PropertyNotFound(String)`: Key or pointer path does not exist.
- `PropertyValueNotOfType { name, not_of_type }`: Path exists but type mismatch.
- `ValueNotOfType(&'static str)`: Internal conversion failure.
- `SerdeJson(serde_json::Error)`: Serialization/Deserialization error.
- `Custom(String)`: Generic error message.
