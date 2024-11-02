# datamorph-rs
A powerful Rust library for transforming data structures using declarative specifications with JSONLogic support. Built for performance, type safety, and extensibility.

[![License](https://img.shields.io/badge/license-apache-blue.svg)](LICENSE)

## Overview

`datamorph-rs` allows you to transform data structures using simple array-based specifications with JSONLogic conditions. It's designed to be easy to use while remaining flexible and extensible.

## Features

- 🚀 Simple, array-based transformation specifications
- 🔧 Built-in transformation functions
- 🎯 Type-safe transformations
- 🔄 Field, concatenation, and split operations
- ⚡ Conditional transformations with JSONLogic
- 📝 Clear error messages

## Quick Start

Add to your `Cargo.toml`:
```toml
[dependencies]
datamorph-rs = "0.1.0"
serde_json = "1.0"
```

Basic example:
```rust
use datamorph_rs::Datamorph;
use serde_json::json;

// Define your transformation spec
let spec = r#"[
    {
        "type": "field",
        "source": "name",
        "target": "fullName",
        "transform": "uppercase",
        "condition": {
            "!!": {"var": "name"}
        }
    },
    {
        "type": "concat",
        "sources": ["city", "country"],
        "target": "location",
        "separator": ", "
    }
]"#;

// Create transformer
let transformer = Datamorph::from_json(spec)?;

// Transform data
let input = json!({
    "name": "john doe",
    "city": "New York",
    "country": "USA"
});

let result: serde_json::Value = transformer.transform(input)?;
println!("Result: {}", serde_json::to_string_pretty(&result)?);
```

## Documentation

- [Usage Guide](USAGE.md) - Detailed usage instructions
- [API Reference](REFERENCE.md) - Complete API documentation
- [Examples](examples/) - Example implementations

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details.

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE](LICENSE) file for details.
