use serde_json::json;
use value_ext::JsonValueExt;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

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
	// Will remove "additionalProperties" (only the frist one, because return false)
	root_value.x_walk(|parent_map, property_name| {
		// --
		if property_name == "type" {
			let val = parent_map.get(property_name).and_then(|val| val.as_str());
			if let Some("object") = val {
				parent_map.remove("additionalProperties");
				return false; // will stop early
			}
		}
		true
	});

	// -- Check
	// the number of "additionalProperties" left
	let mut marker_count = 0;
	// Will remove "additionalProperties"
	root_value.x_walk(|_parent_map, property_name| {
		if property_name == "additionalProperties" {
			marker_count += 1;
		}
		true
	});
	assert_eq!(1, marker_count); // only 1 was removed, as callback returned false

	Ok(())
}
