# Usage Guide

## Table of Contents

1. [Installation](#installation)
2. [Basic Usage](#basic-usage)
3. [Transform Types](#transform-types)
4. [Conditions](#conditions)
5. [Error Handling](#error-handling)

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
datamorph-rs = "0.1.0"
serde_json = "1.0"
```

## Basic Usage

### Simple Transform
```rust
use datamorph_rs::Datamorph;
use serde_json::json;

let spec = r#"[
    {
        "type": "field",
        "source": "name",
        "target": "fullName",
        "transform": "uppercase"
    }
]"#;

let transformer = Datamorph::from_json(spec)?;
let result: serde_json::Value = transformer.transform(input)?;
```

## Transform Types

### Field Transform
```json
{
    "type": "field",
    "source": "user.name",
    "target": "profile.fullName",
    "transform": "uppercase"
}
```

### Concatenation
```json
{
    "type": "concat",
    "sources": ["city", "country"],
    "target": "location",
    "separator": ", "
}
```

### Split
```json
{
    "type": "split",
    "source": "fullAddress",
    "separator": ",",
    "targets": {
        "street": { "index": 0 },
        "city": { "index": 1 }
    }
}
```

## Conditions

Transforms can include JSONLogic conditions:

```json
{
    "type": "field",
    "source": "name",
    "target": "fullName",
    "transform": "uppercase",
    "condition": {
        "and": [
            {"!!": {"var": "name"}},
            {"==": [{"var": "type"}, "person"]}
        ]
    }
}
```

## Error Handling

```rust
use datamorph_rs::{Datamorph, Error};

match Datamorph::from_json(spec) {
    Ok(transformer) => {
        match transformer.transform(input) {
            Ok(result) => println!("Success: {}", 
                serde_json::to_string_pretty(&result)?),
            Err(e) => eprintln!("Transform error: {}", e),
        }
    },
    Err(e) => eprintln!("Failed to parse spec: {}", e),
}
```

## Best Practices

1. **Specification Organization**
   - Use array format for clear transform ordering
   - Group related transforms together
   - Use meaningful field names

2. **Conditions**
   - Use JSONLogic for complex conditions
   - Validate field existence
   - Group conditions logically

3. **Error Handling**
   - Always handle potential errors
   - Validate specifications
   - Consider error recovery strategies

4. **Performance**
   - Order transforms efficiently
   - Use appropriate conditions
   - Consider batch processing