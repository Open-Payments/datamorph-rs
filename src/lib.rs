use std::collections::HashMap;
use serde::Deserialize;
use serde_json::{Value, Map};
use jsonlogic_rs::apply;

mod error;
pub use error::Error;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum TransformType {
    #[serde(rename = "field")]
    Field {
        source: String,
        target: String,
        #[serde(default)]
        transform: Option<String>,
    },
    #[serde(rename = "concat")]
    Concat {
        sources: Vec<String>,
        target: String,
        #[serde(default)]
        separator: Option<String>,
    },
    #[serde(rename = "split")]
    Split {
        source: String,
        targets: HashMap<String, SplitTarget>,
        #[serde(default)]
        separator: Option<String>,
    },
}

#[derive(Debug, Deserialize)]
pub struct Transform {
    #[serde(flatten)]
    pub transform_type: TransformType,
    #[serde(default)]
    pub condition: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct SplitTarget {
    pub index: usize,
    #[serde(default)]
    pub transform: Option<String>,
}

pub struct Datamorph {
    transforms: Vec<Transform>,
}

impl Datamorph {
    pub fn from_json(spec: &str) -> Result<Self, Error> {
        let transforms: Vec<Transform> = serde_json::from_str(spec)?;
        Ok(Self { transforms })
    }

    pub fn transform<T, U>(&self, input: T) -> Result<U, Error>
    where
        T: serde::Serialize,
        U: for<'de> serde::Deserialize<'de>,
    {
        let input = serde_json::to_value(input)?;
        let mut output = Value::Object(Map::new());

        for transform in &self.transforms {
            if let Some(result) = transform.apply(&input)? {
                self.merge_values(&mut output, result);
            }
        }

        Ok(serde_json::from_value(output)?)
    }

    fn merge_values(&self, target: &mut Value, source: Value) {
        if let (Some(target_obj), Some(source_obj)) = (target.as_object_mut(), source.as_object()) {
            for (key, value) in source_obj {
                match target_obj.get_mut(key.as_str()) {  // Using as_str() for String
                    Some(existing) => self.merge_values(existing, value.clone()),
                    None => { target_obj.insert(key.clone(), value.clone()); }
                }
            }
        }
    }
}

impl Transform {
    pub fn apply(&self, input: &Value) -> Result<Option<Value>, Error> {
        if !self.check_condition(input)? {
            return Ok(None);
        }

        match &self.transform_type {
            TransformType::Field { source, target, transform } => {
                let value = self.get_value(input, source)?;
                let transformed = self.apply_transform(value.clone(), transform)?;
                Ok(Some(self.set_value_at_path(target, transformed)))
            },
            TransformType::Concat { sources, target, separator } => {
                let values: Vec<String> = sources.iter()
                    .filter_map(|s| self.get_value(input, s).ok())
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
                
                let result = values.join(separator.as_deref().unwrap_or(""));
                Ok(Some(self.set_value_at_path(target, Value::String(result))))
            },
            TransformType::Split { source, targets, separator } => {
                let value = self.get_value(input, source)?;
                let str_value = value.as_str()
                    .ok_or_else(|| Error::Transform("Source must be string".into()))?;
                
                let parts: Vec<&str> = str_value.split(separator.as_deref().unwrap_or(""))
                    .collect();
                
                let mut result = Map::new();
                for (target_path, split_target) in targets {
                    if let Some(part) = parts.get(split_target.index) {
                        let value = self.apply_transform(
                            Value::String(part.to_string()),
                            &split_target.transform
                        )?;
                        result.insert(target_path.clone(), value);
                    }
                }
                
                Ok(Some(Value::Object(result)))
            },
        }
    }

    fn check_condition(&self, input: &Value) -> Result<bool, Error> {
        match &self.condition {
            Some(condition) => {
                apply(condition, input)
                    .map_err(|e| Error::Logic(e.to_string()))?
                    .as_bool()
                    .ok_or_else(|| Error::Logic("Condition must evaluate to boolean".into()))
            },
            None => Ok(true),
        }
    }

    fn get_value<'a>(&self, input: &'a Value, path: &str) -> Result<&'a Value, Error> {
        path.split('.')
            .try_fold(input, |value, key| {
                value.get(key).ok_or_else(|| Error::MissingField(path.to_string()))
            })
    }

    fn apply_transform(&self, value: Value, transform: &Option<String>) -> Result<Value, Error> {
        match transform {
            Some(transform) => match transform.as_str() {
                "uppercase" => Ok(Value::String(
                    value.as_str()
                        .ok_or_else(|| Error::Transform("Expected string".into()))?
                        .to_uppercase()
                )),
                "lowercase" => Ok(Value::String(
                    value.as_str()
                        .ok_or_else(|| Error::Transform("Expected string".into()))?
                        .to_lowercase()
                )),
                _ => Err(Error::Transform(format!("Unknown transform: {}", transform))),
            },
            None => Ok(value),
        }
    }

    fn set_value_at_path(&self, path: &str, value: Value) -> Value {
        let parts: Vec<&str> = path.split('.').collect();
        let mut result = Map::new();
        let mut current = &mut result;

        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                current.insert((*part).to_string(), value.clone());
            } else {
                current.insert((*part).to_string(), Value::Object(Map::new()));
                current = current.get_mut(*part)  // Using *part directly as it's already &str
                    .unwrap()
                    .as_object_mut()
                    .unwrap();
            }
        }

        Value::Object(result)
    }
}
