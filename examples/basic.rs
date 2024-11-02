use datamorph_rs::Datamorph;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let spec = r#"[
        {
            "type": "field",
            "source": "name",
            "target": "fullName",
            "transform": "uppercase",
            "condition": {
                "==": [{"var": "type"}, "person"]
            }
        },
        {
            "type": "concat",
            "sources": ["city", "country"],
            "target": "location",
            "separator": ", "
        }
    ]"#;

    let input = json!({
        "name": "John Doe",
        "type": "person",
        "city": "New York",
        "country": "USA"
    });

    let transformer = Datamorph::from_json(spec)?;
    let result: serde_json::Value = transformer.transform(input)?;
    
    println!("Transformed: {}", serde_json::to_string_pretty(&result)?);
    
    Ok(())
}