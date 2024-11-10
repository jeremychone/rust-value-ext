# Value Extension Traits

An extension trait for working with JSON values in a more convenient way.
`JsonValueExt` offers convenient methods for interacting with `serde_json::Value` objects,
simplifying tasks like getting, taking, inserting, traversing, and pretty-printing JSON data
while ensuring type safety with Serde's serialization and deserialization.

## Provided Methods

- **`x_get`**: Returns a value of a specified type `T` from a JSON object using either a direct name or a pointer path.
- **`x_take`**: Takes a value from a JSON object using a specified name or pointer path, replacing it with `Null`.
- **`x_insert`**: Inserts a value of type `T` into a JSON object at the specified name or pointer path, creating any missing objects along the way.
- **`x_walk`**: Traverses all properties within the JSON value tree, applying a user-provided callback function to each property.
- **`x_pretty`**: Returns a pretty-printed string representation of the JSON value.

## Usage

This trait is intended to be used with `serde_json::Value` objects. It is particularly
useful when you need to manipulate JSON structures dynamically or when the structure
of the JSON is not known at compile time.

```rust
use serde_json::{Value, Map};
use serde::de::DeserializeOwned;
use serde::Serialize;
use your_crate::JsonValueExt;
fn example_usage(json: &mut Value) -> Result<(), Box<dyn std::error::Error>> {
    // Get a value from JSON
    let name: String = json.x_get("/name")?;
    // Take a value from JSON, replacing it with `Null`
    let age: u32 = json.x_take("age")?;
    // Insert a new value into JSON
    json.x_insert("city", "New York")?;
    // Walk through the JSON properties
    json.x_walk(|parent_map, property_name| {
        println!("Property: {}", property_name);
        true // Continue traversal
    });
    // Get a pretty-printed JSON string
    let pretty_json = json.x_pretty()?;
    println!("{}", pretty_json);
    Ok(())
}
```

This trait enhances the `serde_json::Value` API by adding more type-safe and convenient
methods for manipulating JSON data in Rust.

