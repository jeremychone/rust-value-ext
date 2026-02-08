# Value Extension Traits

[LLM API Reference](doc/for-llm/api-reference-for-llm.md)

`value-ext` is a Rust crate that provides the `JsonValueExt` extension trait for `serde_json::Value`. It offers a more ergonomic and type-safe way to interact with JSON data, simplifying common tasks like getting, taking, inserting, and traversing JSON structures using direct keys or JSON Pointer paths.

## Methods Overview

- **`x_get<T>`**: Returns an owned value of type `T`.
- **`x_get_as<T>`**: Returns a reference or copy (via `AsType`) to avoid unnecessary allocations.
- **`x_get_str`, `x_get_i64`, `x_get_f64`, `x_get_bool`**: Type-specific shortcuts for `x_get_as`.
- **`x_insert`**: Inserts a value at a path, creating parent objects if necessary.
- **`x_take`**: Replaces the value at a path with `Null` and returns the original.
- **`x_remove`**: Removes the property entirely and returns the value.
- **`x_merge`**: Performs a shallow merge of another JSON object.
- **`x_walk`**: Traverses the JSON tree with a callback `(parent_map, key) -> bool`.
- **`x_pretty`**: Returns a formatted JSON string.

This trait enhances the `serde_json::Value` API by providing a more fluid interface for dynamic JSON manipulation in Rust.

## Key Features

- **Ergonomic Getters**: Retrieve values as owned types (`x_get`) or zero-copy references (`x_get_as`, `x_get_str`, etc.).
- **Pointer Support**: Use standard JSON Pointers (e.g., `/path/to/value`) or direct property names.
- **Mutations**: Easily insert (`x_insert`), take (`x_take`), or remove (`x_remove`) values.
- **Deep Traversal**: Walk through the entire JSON tree with `x_walk`.
- **Pretty Printing**: Convenient `x_pretty` method for debugging and logging.

## Usage

Add `value-ext` to your `Cargo.toml` and import the trait.

```rust
use serde_json::{json, Value};
use value_ext::JsonValueExt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut data = json!({
        "project": "value-ext",
        "settings": {
            "retries": 3
        }
    });

    // Get a value using a JSON Pointer
    let retries: i64 = data.x_get("/settings/retries")?;
    assert_eq!(retries, 3);

    // Get a string reference (zero-copy)
    let name = data.x_get_str("project")?;
    assert_eq!(name, "value-ext");

    // Insert a new nested value (creates parents if missing)
    data.x_insert("/settings/timeout", 30)?;

    // Remove a value and return it
    let project: String = data.x_remove("project")?;

    // Pretty print
    println!("{}", data.x_pretty()?);

    Ok(())
}
```

