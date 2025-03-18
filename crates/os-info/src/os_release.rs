// SPDX-FileCopyrightText: Copyright © 2025 AerynOS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Conversion support for os-release files
//!
//! This module provides functionality to convert OSInfo into os-release format
//! for system compatibility purposes.

use std::collections::HashMap;
use std::fmt::Display;

use crate::OSInfo;

/// Represents a parsed os-release file with strongly typed standard fields
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OsRelease {
    /// The name of the operating system
    pub name: String,
    /// The lower case string identifying the operating system
    pub id: String,
    /// The version ID string for the operating system
    pub version_id: String,
    /// The pretty operating system version string
    pub version: String,
    /// The pretty operating system name, quoted if it contains spaces
    pub pretty_name: String,
    /// Space-separated list of operating system identifiers that this OS is based on
    pub id_like: Option<String>,
    /// Home website URL
    pub home_url: Option<String>,
    /// Documentation URL
    pub documentation_url: Option<String>,
    /// Support URL
    pub support_url: Option<String>,
    /// Bug reporting URL
    pub bug_report_url: Option<String>,
    /// Additional non-standard fields
    pub extra_fields: HashMap<String, String>,
}

impl OsRelease {
    /// Creates a new OsRelease with empty required fields
    pub fn new(
        name: String,
        id: String,
        version_id: String,
        version: String,
        pretty_name: String,
    ) -> Self {
        Self {
            name,
            id,
            version_id,
            version,
            pretty_name,
            id_like: None,
            home_url: None,
            documentation_url: None,
            support_url: None,
            bug_report_url: None,
            extra_fields: HashMap::new(),
        }
    }
}

impl Display for OsRelease {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write required fields
        writeln!(f, "NAME={}", shell_escape(&self.name))?;
        writeln!(f, "ID={}", shell_escape(&self.id))?;
        writeln!(f, "VERSION_ID={}", shell_escape(&self.version_id))?;
        writeln!(f, "VERSION={}", shell_escape(&self.version))?;
        writeln!(f, "PRETTY_NAME={}", shell_escape(&self.pretty_name))?;

        // Write optional fields if present
        if let Some(ref id_like) = self.id_like {
            writeln!(f, "ID_LIKE={}", shell_escape(id_like))?;
        }
        if let Some(ref url) = self.home_url {
            writeln!(f, "HOME_URL={}", shell_escape(url))?;
        }
        if let Some(ref url) = self.documentation_url {
            writeln!(f, "DOCUMENTATION_URL={}", shell_escape(url))?;
        }
        if let Some(ref url) = self.support_url {
            writeln!(f, "SUPPORT_URL={}", shell_escape(url))?;
        }
        if let Some(ref url) = self.bug_report_url {
            writeln!(f, "BUG_REPORT_URL={}", shell_escape(url))?;
        }

        // Write any extra fields
        let mut extra: Vec<_> = self.extra_fields.iter().collect();
        extra.sort_by(|(a, _), (b, _)| a.cmp(b));
        for (key, value) in extra {
            writeln!(f, "{}={}", key, shell_escape(value))?;
        }

        Ok(())
    }
}

/// Convert an OSInfo into an os-release format
impl From<&OSInfo> for OsRelease {
    fn from(info: &OSInfo) -> Self {
        let mut release = OsRelease::new(
            info.metadata.identity.name.clone(),
            info.metadata.identity.id.clone(),
            info.metadata.version.short.clone(),
            info.metadata.version.full.clone(),
            info.metadata.identity.display.clone(),
        );

        // Set optional fields
        release.id_like = info.metadata.identity.id_like.clone();

        // Map website URLs based on their scope
        for site in info.resources.websites.values() {
            match site.scope {
                crate::WebsiteScope::Home => {
                    release.home_url = Some(site.url.clone());
                }
                crate::WebsiteScope::Support => {
                    release.support_url = Some(site.url.clone());
                }
                crate::WebsiteScope::BugTracker => {
                    release.bug_report_url = Some(site.url.clone());
                }
                crate::WebsiteScope::Documentation => {
                    release.documentation_url = Some(site.url.clone());
                }
                crate::WebsiteScope::PrivacyPolicy => {
                    release
                        .extra_fields
                        .insert("PRIVACY_POLICY_URL".to_string(), site.url.clone());
                }
                crate::WebsiteScope::TermsOfService => {
                    release
                        .extra_fields
                        .insert("TERMS_OF_SERVICE_URL".to_string(), site.url.clone());
                }
                crate::WebsiteScope::SecurityPolicy => {
                    release
                        .extra_fields
                        .insert("SECURITY_POLICY_URL".to_string(), site.url.clone());
                }
                _ => {}
            }
        }

        // Add security contact to extra fields if available
        if let Some(contact) = &info.security_contact {
            if let Some(email) = &contact.email {
                release
                    .extra_fields
                    .insert("SECURITY_CONTACT".to_string(), email.clone());
            }
        }

        release
    }
}

/// Escape special characters for shell compatibility
fn shell_escape(s: &str) -> String {
    format!("\"{}\"", s.replace('\"', "\\\""))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::load_os_info;

    #[test]
    fn test_os_release_conversion() {
        let sample = include_str!("../../../sample.json");
        let info = load_os_info(sample).unwrap();
        let release = OsRelease::from(&info);

        assert_eq!(release.name, "AerynOS");
        assert_eq!(release.id, "aerynos");
        assert_eq!(release.version_id, "0.25.1");
        assert_eq!(release.pretty_name, "AerynOS 0.25.1");
        assert_eq!(release.id_like, Some("linux".to_string()));
    }

    #[test]
    fn test_os_release_format() {
        let release = OsRelease::new(
            "Test OS".to_string(),
            "testos".to_string(),
            "1.0".to_string(),
            "1.0.0".to_string(),
            "Test OS 1.0".to_string(),
        );

        let output = release.to_string();
        assert!(output.contains("NAME=\"Test OS\"\n"));
        assert!(output.contains("ID=\"testos\"\n"));
        assert!(output.contains("VERSION_ID=\"1.0\"\n"));
    }

    #[test]
    fn test_shell_escape() {
        assert_eq!(shell_escape("simple"), "\"simple\"");
        assert_eq!(shell_escape("with \"quotes\""), "\"with \\\"quotes\\\"\"");
    }

    #[test]
    fn test_extra_fields() {
        let mut release = OsRelease::new(
            "Test".to_string(),
            "test".to_string(),
            "1".to_string(),
            "1.0".to_string(),
            "Test 1.0".to_string(),
        );
        release
            .extra_fields
            .insert("CUSTOM_FIELD".to_string(), "value".to_string());

        let output = release.to_string();
        assert!(output.contains("CUSTOM_FIELD=\"value\"\n"));
    }
}
