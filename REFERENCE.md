# API Reference

## Core Types

### Datamorph

Main entry point for transformations.

```rust
pub struct Datamorph {
    transforms: Vec<Transform>,
}

impl Datamorph {
    /// Create a new instance from JSON specification
    pub fn from_json(spec: &str) -> Result<Self, Error>

    /// Transform input data according to specification
    pub fn transform<T, U>(&self, input: T) -> Result<U, Error>
    where
        T: Serialize,
        U: DeserializeOwned
}
```

### Transform Types

```rust
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum TransformType {
    #[serde(rename = "field")]
    Field {
        source: String,
        target: String,
        transform: Option<String>,
    },
    #[serde(rename = "concat")]
    Concat {
        sources: Vec<String>,
        target: String,
        separator: Option<String>,
    },
    #[serde(rename = "split")]
    Split {
        source: String,
        targets: HashMap<String, SplitTarget>,
        separator: Option<String>,
    },
}

#[derive(Debug, Deserialize)]
pub struct Transform {
    #[serde(flatten)]
    pub transform_type: TransformType,
    pub condition: Option<Value>,
}
```

### Error Types

```rust
pub enum Error {
    /// JSON error
    Json(serde_json::Error),

    /// Transform error
    Transform(String),

    /// Logic error
    Logic(String),

    /// Missing field
    MissingField(String),
}
```

## Specification Format

### Field Transform
```json
{
    "type": "field",
    "source": "sourceField",
    "target": "targetField",
    "transform": "uppercase",
    "condition": {
        "!!": {"var": "sourceField"}
    }
}
```

### Concatenation Transform
```json
{
    "type": "concat",
    "sources": ["field1", "field2"],
    "target": "combinedField",
    "separator": ", ",
    "condition": {
        "and": [
            {"!!": {"var": "field1"}},
            {"!!": {"var": "field2"}}
        ]
    }
}
```

### Split Transform
```json
{
    "type": "split",
    "source": "fullAddress",
    "separator": ",",
    "targets": {
        "street": {
            "index": 0,
            "transform": "uppercase"
        },
        "city": {
            "index": 1
        }
    }
}
```

## JSONLogic Conditions

Common condition patterns:

```json
// Field exists
{"!!": {"var": "fieldName"}}

// Value equals
{"==": [{"var": "field"}, "value"]}

// Multiple conditions (AND)
{"and": [
    {"!!": {"var": "field1"}},
    {"==": [{"var": "field2"}, "value"]}
]}

// Multiple conditions (OR)
{"or": [
    {"!!": {"var": "field1"}},
    {"!!": {"var": "field2"}}
]}
```