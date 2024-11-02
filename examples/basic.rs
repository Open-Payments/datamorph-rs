use datamorph_rs::Datamorph;
use serde_json::json;

fn main() -> anyhow::Result<()> {
    // Define transformation spec
    let spec = r#"{
        "mappings": {
            "name": {
                "target": "fullName",
                "transform": "uppercase"
            },
            "age": {
                "target": "userAge",
                "transform": "toString"
            }
        }
    }"#;

    // Create input data
    let input = json!({
        "name": "john doe",
        "age": 30
    });

    // Initialize transformer and apply transformation
    let datamorph = Datamorph::from_json(spec)?;
    let result: serde_json::Value = datamorph.transform(input)?;

    println!("Transformed data: {}", result);
    Ok(())
}