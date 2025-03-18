# OS Info

A standardized format for describing operating system metadata, capabilities, and resources.

## Overview

This project provides JSON schemas and examples for representing comprehensive information about operating systems in a structured, machine-readable format. The format includes:

- System metadata (identity, maintainers, versioning)
- Technical capabilities (kernel, platform, update strategy)
- System composition and core technologies
- Boot and firmware features
- Available resources (websites, documentation, social channels)

While other tools like lsb_release and os-release exist for basic OS identification, this project aims to provide a more comprehensive schema that includes capabilities, resources, and system composition details that those tools don't cover. Where lsb_release focuses on Linux compatibility information and os-release provides basic identification, os-info standardizes metadata about an OS's full feature set and ecosystem.



## Schema Structure

The project contains two main schemas:

- [`os-info.schema.json`](schema/0.1/os-info.schema.json) - The main schema for OS descriptions
- [`technology-capabilities.schema.json`](schema/0.1/technology-capabilities.schema.json) - Schema for describing individual technologies

## Usage

1. Create a JSON file describing your OS using the schema at:
```
https://raw.githubusercontent.com/AerynOS/os-info/refs/heads/main/schema/0.1/os-info.schema.json
```

2. For each core technology, create a capabilities file using:
```
https://raw.githubusercontent.com/AerynOS/os-info/refs/heads/main/schema/0.1/technology-capabilities.schema.json
```

See [`sample.json`](sample.json) for a complete example implementation.

## Key Features

- Standardized version and identity tracking
- Support for tracking historical identity changes
- Detailed system composition and capabilities
- Structured resource links (documentation, social, funding)
- Atomic update and rollback information
- Platform and architecture specifications

## Structure

```
.
├── crates/
│   └── os-info/           # Rust library implementation
├── schema/
│   └── 0.1/
│       ├── os-info.schema.json
│       └── technology-capabilities.schema.json
├── technologies/
│   └── *.json            # Technology capability definitions
└── sample.json           # Example OS info file
```

## Rust Library

The project includes a Rust library for working with os-info files. The library provides:

- Type-safe structs representing the full OS info schema
- Serialization/deserialization via serde
- Helper functions for loading files
- Error handling for common failure cases

### Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
os-info = "0.1"
```

Basic example:
```rust
use os_info::{load_os_info_from_path, load_technology_from_path};

// Load and parse an OS info file
let os_info = load_os_info_from_path("os-info.json")?;

// Access the parsed data
println!("OS Name: {}", os_info.metadata.identity.name);
println!("Version: {}", os_info.metadata.version.full);

// Load technology capabilities
let tech = load_technology_from_path("technologies/moss.json")?;
println!("Technology: {}", tech.name);
```

The library provides strongly-typed structs for all schema components, making it easy to work with OS info data in a type-safe way.

## Contributing

Contributions are welcome! Please feel free to submit pull requests to:

- Add new schema fields for describing OS capabilities
- Improve schema validation rules
- Add examples for other operating systems
- Enhance documentation

## License

`os-info` is available under the terms of the [MPL-2.0](https://spdx.org/licenses/MPL-2.0.html)
