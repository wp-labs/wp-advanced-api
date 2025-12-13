# WP Advanced API

[![CI](https://github.com/wp-labs/wp-advanced-api/workflows/CI/badge.svg)](https://github.com/wp-labs/wp-advanced-api/actions/workflows/ci.yml)
[![Release](https://github.com/wp-labs/wp-advanced-api/workflows/Release/badge.svg)](https://github.com/wp-labs/wp-advanced-api/actions/workflows/release.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A Rust-based enrichment API framework for the Warp Flow ecosystem, designed for advanced data processing and enrichment in WordPress data pipelines.

## Overview

WP Advanced API is part of the Warp Flow data processing framework (similar to Apache Beam or Flink). It provides a pluggable architecture for enriching WordPress data as it flows through processing pipelines.

## Project Structure

This is a Rust workspace containing two main crates:

- **wp-enrich-api**: Core enrichment API implementing traits for data field enrichment
- **wp-ctrl-api**: Control API crate (currently in development)

## Features

- 🚀 **Pluggable Enrichment**: Implement custom enrichment logic through well-defined traits
- 📦 **Workspace Management**: Efficient dependency management across multiple crates
- 🔧 **Build Automation**: Automated build, lint, and release processes with GXL
- 🔄 **CI/CD Integration**: GitHub Actions for continuous integration and deployment
- 📝 **Semantic Versioning**: Automated version management and releases

## Quick Start

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- Cargo (included with Rust)
- GXL (for build automation)

### Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
wp-enrich-api = "0.0.2"
wp-model-core = { git = "https://github.com/wp-labs/wp-model-core.git" }
```

### Usage Example

```rust
use wp_enrich_api::{EnrichingAble, EnrichLibAble};

struct MyEnricher;

impl EnrichingAble for MyEnricher {
    fn enrich(&self, target: &str, needs: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Your enrichment logic here
        Ok(format!("Enriched data for {} with {}", target, needs))
    }
}

struct MyEnrichRegistry;

impl EnrichLibAble for MyEnrichRegistry {
    fn lookup(&self, field: &str) -> Option<Box<dyn EnrichingAble>> {
        match field {
            "my_field" => Some(Box::new(MyEnricher)),
            _ => None,
        }
    }
}
```

## API Documentation

### Core Traits

#### `EnrichingAble`

Trait for implementing data enrichment logic:

```rust
pub trait EnrichingAble: Send + Sync {
    fn enrich(&self, target: &str, needs: &str) -> Result<String, Box<dyn std::error::Error>>;
}
```

#### `EnrichLibAble`

Trait for enrichment registry and lookup:

```rust
pub trait EnrichLibAble: Send + Sync {
    fn lookup(&self, field: &str) -> Option<Box<dyn EnrichingAble>>;
}
```

## Development

### Building

```bash
# Using GXL (recommended)
gxl run build

# Or using Cargo directly
cargo build
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with coverage
cargo tarpaulin --out Html
```

### Linting

```bash
# Run all lints
gxl run lint

# Or manually
cargo clippy -- -D warnings
cargo fmt -- --check
```

## Build Automation

This project uses GXL for build automation. The following flows are available:

- `build`: Build all crates
- `lint`: Run code formatting and clippy
- `bump-version`: Bump semantic version
- `commit`: Commit changes with conventional commits
- `release`: Create a new release

See `.run.gxl` for detailed flow definitions.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and linting (`gxl run lint && cargo test`)
5. Commit your changes with conventional commit format
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Commit Convention

We follow conventional commits:

- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `style:` for code formatting changes
- `refactor:` for code refactoring
- `test:` for adding or updating tests
- `chore:` for maintenance changes

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Related Projects

- [wp-model-core](https://github.com/wp-labs/wp-model-core) - Core models and types
- [wp-source-api](https://github.com/wp-labs/wp-source-api) - Source API implementation
- [wp-sink-api](https://github.com/wp-labs/wp-sink-api) - Sink API implementation

## Version History

- **0.0.2** - Current version
  - Initial release with enrichment API framework
  - Workspace structure setup
  - CI/CD pipeline integration

## Support

For support and questions:

- Open an [Issue](https://github.com/wp-labs/wp-advanced-api/issues)
- Check the [Wiki](https://github.com/wp-labs/wp-advanced-api/wiki)
- Join our discussions

---

Made with ❤️ by the WP Labs team