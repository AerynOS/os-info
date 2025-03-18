// SPDX-FileCopyrightText: Copyright © 2025 AerynOS Developers
//
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the capabilities of a technology, including version info and related links
#[derive(Debug, Serialize, Deserialize)]
pub struct TechnologyCapabilities {
    /// Version string for the technology capabilities specification
    #[serde(rename = "technology-capabilities-version")]
    pub version: String,
    /// Name of the technology
    pub name: String,
    /// Description of the technology's capabilities
    pub description: String,
    /// Collection of related links, mapped by link ID
    pub links: HashMap<String, TechnologyLink>,
}

/// Represents a link related to a technology capability
#[derive(Debug, Serialize, Deserialize)]
pub struct TechnologyLink {
    /// The type of link (e.g. "documentation", "source", etc)
    #[serde(rename = "type")]
    pub link_type: String,
    /// Category the link belongs to
    pub category: String,
    /// URL of the link
    pub url: String,
}
