use serde_json::json;
use value_ext::{AsType, JsonValueExt};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[test]
fn test_value_insert_ok() -> Result<()> {
	// -- Setup & Fixtures
	let mut value = json!({"tokens": 3});
	let fx_node_value = "hello";

	// -- Exec
	value.x_insert("/happy/word", fx_node_value)?;

	// -- Check
	let actual_value: String = value.x_get("/happy/word")?;
	assert_eq!(actual_value.as_str(), fx_node_value);

	Ok(())
}

#[test]
fn test_value_walk_ok() -> Result<()> {
	// -- Setup & Fixtures
	let mut root_value = json!(
	{
		"tokens": 3,
		"schema": {
			"type": "object",
			"additionalProperties": false,
			"properties": {
				"all_models": {
					"type": "array",
					"items": {
						"type": "object",
						"additionalProperties": false,
						"properties": {
							"maker": { "type": "string" },
							"model_name": { "type": "string" }
						},
						"required": ["maker", "model_name"]
					}
				}
			},
			"required": ["all_models"]
		}
	});

	// -- Exec
	// Will remove "additionalProperties" (only the first one, because callback returns false)
	root_value.x_walk(|parent_map, property_name| {
		if property_name == "type" {
			let val = parent_map.get(property_name).and_then(|v| v.as_str());
			if let Some("object") = val {
				parent_map.remove("additionalProperties");
				return false; // stop early
			}
		}
		true
	});

	// -- Check
	let mut marker_count = 0;
	root_value.x_walk(|_parent_map, property_name| {
		if property_name == "additionalProperties" {
			marker_count += 1;
		}
		true
	});
	assert_eq!(1, marker_count); // only one was removed

	Ok(())
}

#[test]
fn test_as_type_for_vec() -> Result<()> {
	// -- Setup & Fixtures: Create a JSON array
	let json_array = json!([ {"a": 1}, {"b": 2} ]);

	// -- Exec: Use the AsType implementation for &Vec<Value>
	let vec_ref: &Vec<serde_json::Value> = <&Vec<serde_json::Value>>::from_value(&json_array)?;

	// -- Check: Validate the length and content
	assert_eq!(vec_ref.len(), 2);

	let first_obj = &vec_ref[0];
	let a_val = first_obj
		.get("a")
		.and_then(|v| v.as_i64())
		.ok_or("Missing 'a' in first element")?;
	assert_eq!(a_val, 1);

	Ok(())
}

#[test]
fn test_as_type_for_vec_str() -> Result<()> {
	// -- Setup & Fixtures: Create a JSON array of strings
	let json_array = json!(["hello", "world"]);

	// -- Exec: Use the AsType implementation for Vec<&str>
	let vec_str: Vec<&str> = <Vec<&str>>::from_value(&json_array)?;

	// -- Check: Validate the length and content
	assert_eq!(vec_str, vec!["hello", "world"]);

	Ok(())
}

#[test]
fn test_x_remove_direct() -> Result<()> {
	// -- Setup & Fixtures: Create a JSON object with a direct key.
	let mut value = json!({"key": "direct_value", "other": 42});
	// -- Exec: Remove the direct key using x_remove.
	let removed: String = value.x_remove("key")?;
	// -- Check: The removed value should equal "direct_value"
	assert_eq!(removed, "direct_value");
	// Also, check that "key" is no longer in the object.
	let obj = value.as_object().ok_or("Expected object after removal")?;
	assert!(!obj.contains_key("key"));
	// "other" remains unchanged.
	let other: i64 = value.x_get("other")?;
	assert_eq!(other, 42);
	Ok(())
}

#[test]
fn test_x_remove_nested() -> Result<()> {
	// -- Setup: Create a nested JSON object.
	let mut value = json!({
		"a": {
			"b": {
				"c": "nested_value",
				"d": "keep_this"
			},
			"e": "direct_in_a"
		},
		"f": "outside"
	});
	// -- Exec: Remove the nested element "c" from "a/b"
	let removed: String = value.x_remove("/a/b/c")?;
	// -- Check: The removed value should equal "nested_value"
	assert_eq!(removed, "nested_value");

	// Now, "a/b" should still have key "d"
	let d: String = value.x_get("/a/b/d")?;
	assert_eq!(d, "keep_this");

	// And key "c" should be missing from "a/b"
	let b = value.pointer("/a/b").ok_or("Expected pointer /a/b not found")?;
	let b_obj = b.as_object().ok_or("Expected object at /a/b")?;
	assert!(!b_obj.contains_key("c"));

	// Other parts of the object remain unchanged.
	let direct_in_a: String = value.x_get("/a/e")?;
	assert_eq!(direct_in_a, "direct_in_a");

	let outside: String = value.x_get("f")?;
	assert_eq!(outside, "outside");

	Ok(())
}

#[test]
fn test_x_merge_ok() -> Result<()> {
	// -- Setup & Fixtures
	let mut value = json!({"a": 1, "b": 2});
	let other = json!({"b": 3, "c": 4});

	// -- Exec
	value.x_merge(other)?;

	// -- Check
	assert_eq!(value["a"], 1);
	assert_eq!(value["b"], 3);
	assert_eq!(value["c"], 4);

	// -- Test Null (should do nothing)
	value.x_merge(serde_json::Value::Null)?;
	assert_eq!(value["b"], 3);

	Ok(())
}
