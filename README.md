# datamorph-rs
A powerful Rust library for transforming data structures using declarative specifications. Built for performance, type safety, and extensibility.

<!-- [![Crates.io](https://img.shields.io/crates/v/datamorph-rs.svg)](https://crates.io/crates/datamorph-rs)
[![Documentation](https://docs.rs/datamorph-rs/badge.svg)](https://docs.rs/datamorph-rs) -->
[![License](https://img.shields.io/badge/license-apache-blue.svg)](LICENSE)

## Overview

`datamorph-rs` allows you to transform data structures using simple JSON-based specifications. It's designed to be easy to use while remaining flexible and extensible.

## Features

- 🚀 Simple, declarative JSON transformation specifications
- 🔧 Built-in transformation functions
- 🎯 Type-safe transformations
- 🔄 Support for multiple transformations
- ⚡ Zero-copy where possible
- 📝 Clear error messages

## Quick Start

Add to your `Cargo.toml`:
```toml
[dependencies]
datamorph-rs = "0.1.0"
```

Basic example:
```rust
use datamorph_rs::Datamorph;
use serde_json::json;

// Define your transformation spec
let spec = r#"{
    "mappings": {
        "name": {
            "target": "fullName",
            "transform": "uppercase"
        }
    }
}"#;

// Create transformer
let transformer = Datamorph::from_json(spec)?;

// Transform data
let input = json!({ "name": "john doe" });
let result: serde_json::Value = transformer.transform(input)?;

assert_eq!(result["fullName"], "JOHN DOE");
```

## Documentation

- [Usage Guide](docs/USAGE.md) - Detailed usage instructions
- [API Reference](docs/REFERENCE.md) - Complete API documentation
- [Examples](examples/) - Example implementations

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details.

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE](LICENSE) file for details.
