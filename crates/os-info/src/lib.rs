// SPDX-FileCopyrightText: Copyright © 2025 AerynOS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Data structures for representing operating system information and metadata.
//!
//! This module provides types for storing and serializing information about an operating system,
//! including system details, versioning, maintainers, and resources.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod schema;
pub use schema::*;
mod technology;
pub use technology::*;
mod os_release;
pub use os_release::*;

/// Top-level structure containing all OS information
#[derive(Debug, Serialize, Deserialize)]
pub struct OSInfo {
    /// Version of the OS info schema
    #[serde(rename = "os-info-version")]
    pub version: String,
    /// Date the OS project was started
    pub start_date: DateTime<Utc>,
    /// Metadata about the OS
    pub metadata: Metadata,
    /// System configuration and details
    pub system: System,
    /// Links to project resources
    pub resources: Resources,
    /// Security contact information
    #[serde(default)]
    pub security_contact: Option<SecurityContact>,
}

/// Metadata about the OS including identity, maintainers and version information
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// Core identity information
    pub identity: Identity,
    /// Map of maintainer groups to lists of maintainers
    pub maintainers: HashMap<String, Vec<Maintainer>>,
    /// Version information
    pub version: VersionInfo,
}

/// Identity information for the OS
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Identity {
    /// Unique identifier for the OS
    pub id: String,
    /// Parent OS this is based on/similar to
    #[serde(rename = "id_like")]
    pub id_like: Option<String>,
    /// Full name of the OS
    pub name: String,
    /// Display name/branding
    pub display: String,
    /// ANSI terminal color code for branding
    pub ansi_color: Option<String>,
    /// Previous identities/names
    pub former_identities: Vec<FormerIdentity>,
}

/// Historical identity information
#[derive(Debug, Serialize, Deserialize)]
pub struct FormerIdentity {
    /// Previous OS identifier
    pub id: String,
    /// Previous OS name
    pub name: String,
    /// When this identity started
    pub start_date: DateTime<Utc>,
    /// When this identity ended
    pub end_date: DateTime<Utc>,
    /// Version when identity was changed
    pub end_version: Option<String>,
    /// Link to announcement of change
    pub announcement: Option<String>,
}

/// Information about a project maintainer
#[derive(Debug, Serialize, Deserialize)]
pub struct Maintainer {
    /// Full name of maintainer
    pub name: String,
    /// Role/position in project
    pub role: MaintainerRole,
    /// Contact email
    pub email: String,
    /// When the maintainer joined the project
    pub start_date: Option<DateTime<Utc>>,
    /// When the maintainer left the project (if applicable)
    pub end_date: Option<DateTime<Utc>>,
}

/// Role types for maintainers
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MaintainerRole {
    /// Project founder
    Founder,
    /// Core maintainer
    Maintainer,
    /// Regular contributor
    Contributor,
}

/// Detailed version information
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionInfo {
    /// Complete version string
    pub full: String,
    /// Short version number
    pub short: String,
    /// Unique build identifier
    pub build_id: String,
    /// Release date
    pub released: DateTime<Utc>,
    /// Link to release announcement
    pub announcement: Option<String>,
    /// Version codename
    pub codename: Option<String>,
}

/// Core system information and configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct System {
    /// System composition details
    pub composition: Composition,
    /// Enabled features
    pub features: Features,
    /// Kernel information
    pub kernel: Kernel,
    /// Platform details
    pub platform: Platform,
    /// Update configuration
    pub update: Update,
}

/// System composition including base systems and technologies
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Composition {
    /// Base systems used
    pub bases: Vec<String>,
    /// Technology components
    pub technology: Technology,
}

/// Core and optional technology components
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Technology {
    /// Required core technologies
    pub core: Vec<String>,
    /// Optional add-on technologies
    pub optional: Vec<String>,
}

/// System feature configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Features {
    /// Atomic update settings
    pub atomic_updates: AtomicUpdates,
    /// Boot configuration
    pub boot: Boot,
    /// Filesystem settings
    pub filesystem: Filesystem,
}

/// Atomic update system configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct AtomicUpdates {
    /// Update strategy used
    pub strategy: String,
    /// Whether rollbacks are supported
    pub rollback_support: bool,
}

/// Boot configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Boot {
    /// Bootloader used
    pub bootloader: String,
    /// Firmware support details
    pub firmware: Firmware,
}

/// Firmware support configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Firmware {
    /// UEFI support
    pub uefi: bool,
    /// Secure Boot support
    pub secure_boot: bool,
    /// Legacy BIOS support
    pub bios: bool,
}

/// Filesystem configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Filesystem {
    /// Default filesystem
    pub default: String,
    /// Supported filesystems
    pub supported: Vec<String>,
}

/// Kernel information
#[derive(Debug, Serialize, Deserialize)]
pub struct Kernel {
    /// Type of kernel
    #[serde(rename = "type")]
    pub kernel_type: String,
    /// Kernel name
    pub name: String,
}

/// Platform architecture information
#[derive(Debug, Serialize, Deserialize)]
pub struct Platform {
    /// CPU architecture
    pub architecture: String,
    /// Architecture variant
    pub variant: String,
}

/// Update strategy configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Update {
    /// Update strategy used
    pub strategy: String,
    /// Update cadence settings
    pub cadence: Cadence,
    /// Update approach used
    pub approach: String,
}

/// Update cadence configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Cadence {
    /// Type of update cadence
    #[serde(rename = "type")]
    pub cadence_type: CadenceType,
    /// Update sync interval
    pub sync_interval: Option<String>,
    /// Day updates sync
    pub sync_day: Option<String>,
    /// Release schedule
    pub release_schedule: Option<String>,
    /// Support timeline
    pub support_timeline: Option<String>,
}

/// Types of update cadence
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CadenceType {
    /// Rolling release
    Rolling,
    /// Fixed point release
    Fixed,
    /// Long-term support release
    Lts,
    /// Point release
    Point,
}

/// Project resources like websites and social media
#[derive(Debug, Serialize, Deserialize)]
pub struct Resources {
    /// Project websites
    pub websites: HashMap<String, Website>,
    /// Social media links
    pub social: HashMap<String, SocialLink>,
    /// Funding platform links
    pub funding: HashMap<String, FundingLink>,
}

/// Website information
#[derive(Debug, Serialize, Deserialize)]
pub struct Website {
    /// Website URL
    pub url: String,
    /// Display name
    pub display_name: String,
    /// Website scope/purpose
    pub scope: WebsiteScope,
}

/// Website scope/purpose
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum WebsiteScope {
    /// Main project homepage
    Home,
    /// General documentation
    Documentation,
    /// User support resources
    Support,
    /// Bug reporting/issue tracking
    BugTracker,
    /// Developer portal
    Developer,
    /// Public-facing website
    Public,
    /// End user documentation
    EndUserDocs,
    /// Developer documentation
    DeveloperDocs,
    /// Privacy policy document
    PrivacyPolicy,
    /// Terms of service/use document
    TermsOfService,
    /// Legal information
    Legal,
    /// Security policy and vulnerability reporting
    SecurityPolicy,
}

/// Social media link
#[derive(Debug, Serialize, Deserialize)]
pub struct SocialLink {
    /// Social media URL
    pub url: String,
    /// Display name
    pub display_name: String,
    /// Platform name
    pub platform: String,
}

/// Funding platform link
#[derive(Debug, Serialize, Deserialize)]
pub struct FundingLink {
    /// Funding page URL
    pub url: String,
    /// Display name
    pub display_name: String,
    /// Platform name
    pub platform: String,
}
/// Security contact information for vulnerability reporting
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SecurityContact {
    /// Email address for security-related communications
    pub email: Option<String>,
    /// PGP key for encrypted vulnerability reports
    pub pgp_key: Option<String>,
    /// Vulnerability disclosure policy
    pub disclosure_policy: Option<String>,
}

#[cfg(test)]
mod tests;
