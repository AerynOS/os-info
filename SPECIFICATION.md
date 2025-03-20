# OS Information Specification

Version 0.1

## Introduction

The OS Information Specification defines a standardized format for describing operating systems and their capabilities. It provides a common way to represent information about an operating system's identity, features, maintainers, resource links, and security policies in a machine-readable format.

## Purpose

This specification serves several purposes:

1. **System Identification**: Provides a standard way for operating systems to identify themselves to applications and tools
2. **Feature Discovery**: Allows software to discover OS capabilities and features
3. **Resource Discoverability**: Centralized definition of project resources like documentation, support channels, and security contacts
4. **System Compatibility**: Enables conversion to standard formats like `os-release` for compatibility with existing tools

## Schema Structure

The schema is organized into several main sections:

### Top-Level Structure

- `os-info-version`: The version of the schema specification (e.g., "0.1")
- `start_date`: The date the OS project was started
- `metadata`: Information about the OS identity, maintainers, and version
- `system`: System configuration, features, and platform details
- `resources`: Links to project resources
- `security_contact`: Contact information for vulnerability reporting

### Metadata Section

The metadata section contains:

- **Identity Information**:
  - `id`: Unique OS identifier
  - `name`: Full OS name
  - `display`: Display name for presentation
  - `ansi_color`: ANSI color escape sequence for terminal branding (optional)
  - `id_like`: Parent OS identifier (optional)
  - `former_identities`: Historical names and identifiers

- **Maintainers**:
  - Organized by group (e.g., "core", "contributors")
  - Each entry includes name, role, email, and dates

- **Version Information**:
  - `full`: Complete version string
  - `short`: Short version number
  - `build_id`: Unique build identifier
  - `released`: Release date
  - `codename`: Version codename (optional)

### System Section

The system section describes:

- **Composition**:
  - `bases`: Base systems used
  - `technology`: Core and optional technology components

- **Features**:
  - `atomic_updates`: Update strategy and rollback support
  - `boot`: Bootloader and firmware support
  - `filesystem`: Default and supported filesystems

- **Kernel Information**:
  - `type`: Type of kernel
  - `name`: Kernel name

- **Platform**:
  - `architecture`: CPU architecture
  - `variant`: Architecture variant

- **Update Strategy**:
  - `strategy`: Update method
  - `cadence`: Release cadence and schedule
  - `approach`: Update approach

### Resources Section

The resources section provides:

- **Websites**: Project websites with scope/purpose identifiers
- **Social**: Social media links
- **Funding**: Funding platform links

### Security Contact

Security contact information includes:

- `email`: Security contact email
- `pgp_key`: PGP key for encrypted communication
- `disclosure_policy`: Vulnerability disclosure policy

## Website Scopes

Website entries include a scope field to identify the purpose of each link:

- `home`: Main project homepage
- `documentation`: General documentation
- `support`: User support resources
- `bug-tracker`: Bug reporting/issue tracking
- `developer`: Developer portal
- `public`: Public-facing website
- `end-user-docs`: End user documentation
- `developer-docs`: Developer documentation
- `privacy-policy`: Privacy policy document
- `terms-of-service`: Terms of service document
- `legal`: Legal information
- `security-policy`: Security policy

## Technology Capabilities

The specification also supports describing technology capabilities in separate files:

```json
{
  "technology-capabilities-version": "1.0",
  "name": "example-technology",
  "description": "Description of the technology",
  "links": {
    "documentation": {
      "type": "website",
      "category": "documentation",
      "url": "https://example.com/docs"
    }
  }
}
```

## OS-Release Compatibility

The schema supports conversion to `os-release` format for compatibility with Linux tools and standards. This is handled through the `OsRelease` structure which maps fields appropriately:

- Standard fields: `NAME`, `ID`, `VERSION_ID`, `VERSION`, `PRETTY_NAME`
- Optional fields: `ID_LIKE`, `HOME_URL`, etc.
- Security and policy URLs are mapped to their corresponding `os-release` fields

## Usage Examples

### Basic OS Information

```json
{
  "os-info-version": "0.1",
  "start_date": "2023-01-01T00:00:00Z",
  "metadata": {
    "identity": {
      "id": "example-os",
      "name": "Example OS",
      "display": "Example OS"
    },
    "maintainers": {
      "core": [
        {
          "name": "Jane Doe",
          "role": "founder",
          "email": "jane@example.com"
        }
      ]
    },
    "version": {
      "full": "1.0.0",
      "short": "1.0",
      "build_id": "12345",
      "released": "2023-05-01T00:00:00Z"
    }
  },
  "system": {
    // System details here
  },
  "resources": {
    // Resource links here
  }
}
```

### Security Contact Information

```json
"security_contact": {
  "email": "security@example.com",
  "pgp_key": "-----BEGIN PGP PUBLIC KEY BLOCK-----\n...\n-----END PGP PUBLIC KEY BLOCK-----",
  "disclosure_policy": "90-day responsible disclosure policy"
}
```

## Benefits of Adoption

- **Standardization**: Provides a common format for OS information
- **Discoverability**: Makes OS resources and capabilities easily discoverable
- **Compatibility**: Enables conversion to standard formats
- **Security**: Standardizes security contact information
- **Tooling**: Supports development of cross-OS tools and utilities

## Implementation

The reference implementation is provided by the `os-info` Rust crate, which includes:

- Data structures for parsing and serializing OS information
- Conversion to `os-release` format
- Schema validation support
- Technology capabilities parsing
