use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{Error, Result};

#[derive(Debug, Clone, Deserialize)]
pub struct TransformSpec {
    pub mappings: HashMap<String, Mapping>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Mapping {
    pub target: String,
    pub transform: Transform,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Transform {
    Single(String),
    Multiple(Vec<String>),
}

impl TransformSpec {
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(|e| Error::SpecParseError(e.to_string()))
    }

    pub fn transform(&self, input: &serde_json::Value) -> Result<serde_json::Value> {
        let mut output = serde_json::Map::new();

        for (field, mapping) in &self.mappings {
            if let Some(value) = input.get(field) {
                let transformed = self.apply_transform(value, &mapping.transform)?;
                output.insert(mapping.target.clone(), transformed);
            }
        }

        Ok(serde_json::Value::Object(output))
    }

    fn apply_transform(&self, value: &serde_json::Value, transform: &Transform) -> Result<serde_json::Value> {
        match transform {
            Transform::Single(func) => self.apply_single_transform(value, func),
            Transform::Multiple(funcs) => {
                let mut result = value.clone();
                for func in funcs {
                    result = self.apply_single_transform(&result, func)?;
                }
                Ok(result)
            }
        }
    }

    fn apply_single_transform(&self, value: &serde_json::Value, func: &str) -> Result<serde_json::Value> {
        match func {
            "uppercase" => self.transform_uppercase(value),
            "lowercase" => self.transform_lowercase(value),
            "toString" => self.transform_to_string(value),
            _ => Err(Error::TransformError(format!("Unknown transform: {}", func))),
        }
    }

    fn transform_uppercase(&self, value: &serde_json::Value) -> Result<serde_json::Value> {
        if let Some(s) = value.as_str() {
            Ok(serde_json::Value::String(s.to_uppercase()))
        } else {
            Err(Error::TransformError("Expected string for uppercase".into()))
        }
    }

    fn transform_lowercase(&self, value: &serde_json::Value) -> Result<serde_json::Value> {
        if let Some(s) = value.as_str() {
            Ok(serde_json::Value::String(s.to_lowercase()))
        } else {
            Err(Error::TransformError("Expected string for lowercase".into()))
        }
    }

    fn transform_to_string(&self, value: &serde_json::Value) -> Result<serde_json::Value> {
        Ok(serde_json::Value::String(value.to_string()))
    }
}