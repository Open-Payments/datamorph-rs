//! datamorph-rs: Data transformation library using declarative specifications

mod error;
mod spec;
mod transform;

pub use error::{Error, Result};
pub use spec::TransformSpec;
// pub use transform::Transform;

/// Main entry point for transformations
#[derive(Debug)]
pub struct Datamorph {
    spec: TransformSpec,
}

impl Datamorph {
    /// Create a new instance from a JSON specification
    pub fn from_json(spec_str: &str) -> Result<Self> {
        let spec = TransformSpec::from_json(spec_str)?;
        Ok(Self { spec })
    }

    /// Transform input data according to the specification
    pub fn transform<T, U>(&self, input: T) -> Result<U>
    where
        T: serde::Serialize,
        U: serde::de::DeserializeOwned,
    {
        let input_value = serde_json::to_value(input)?;
        let transformed = self.spec.transform(&input_value)?;
        Ok(serde_json::from_value(transformed)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_basic_transform() -> Result<()> {
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

        let input = json!({
            "name": "john doe",
            "age": 30
        });

        let datamorph = Datamorph::from_json(spec)?;
        let result: serde_json::Value = datamorph.transform(input)?;

        assert_eq!(result["fullName"], "JOHN DOE");
        assert_eq!(result["userAge"], "30");

        Ok(())
    }
}
