// SPDX-FileCopyrightText: Copyright © 2025 AerynOS Developers
//
// SPDX-License-Identifier: MPL-2.0

use super::*;
use chrono::Utc;
use std::fs;

const SAMPLE_PATH: &str = "../../sample.json";
const TECHNOLOGIES_PATH: &str = "../../technologies";

/// Tests that sample.json can be parsed successfully
#[test]
fn test_parse_sample_os_info() {
    let os_info = load_os_info_from_path(SAMPLE_PATH).unwrap();

    assert_eq!(os_info.version, "0.1");
    assert_eq!(os_info.metadata.identity.id, "aerynos");
    assert_eq!(os_info.metadata.identity.name, "AerynOS");

    // Verify former identity
    let former = &os_info.metadata.identity.former_identities[0];
    assert_eq!(former.id, "serpentos");
    assert_eq!(former.name, "Serpent OS");

    // Verify core technologies
    let core_tech = &os_info.system.composition.technology.core;
    assert!(core_tech.contains(&"boulder".to_string()));
    assert!(core_tech.contains(&"moss".to_string()));
}

/// Tests all technology capability files can be parsed
#[test]
fn test_parse_technologies() {
    let paths = fs::read_dir(TECHNOLOGIES_PATH).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let tech = load_technology_from_path(path).unwrap();

        // Verify basic fields are present
        assert!(!tech.name.is_empty());
        assert!(!tech.description.is_empty());
        assert!(!tech.links.is_empty());

        // Check for documentation link
        assert!(tech.links.contains_key("documentation"));
    }
}

#[test]
fn test_maintainer_role_serialization() {
    let maintainer = Maintainer {
        name: "Test User".to_string(),
        role: MaintainerRole::Founder,
        email: "test@example.com".to_string(),
        start_date: Some(
            chrono::DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
        ),
        end_date: None,
    };

    let serialized = serde_json::to_string(&maintainer).unwrap();
    let deserialized: Maintainer = serde_json::from_str(&serialized).unwrap();

    assert_eq!(maintainer.name, deserialized.name);
    assert!(matches!(deserialized.role, MaintainerRole::Founder));
    assert_eq!(maintainer.start_date, deserialized.start_date);
    assert_eq!(maintainer.end_date, deserialized.end_date);
}

#[test]
fn test_security_contact_and_policy_links() {
    let json = r#"
{
    "os-info-version": "0.1",
    "start_date": "2023-01-01T00:00:00Z",
    "metadata": {
        "identity": {
            "id": "test-os",
            "name": "Test OS",
            "display": "Test OS",
            "former_identities": []
        },
        "maintainers": {
            "core": [
                {
                    "name": "Test User",
                    "role": "founder",
                    "email": "test@example.com"
                }
            ]
        },
        "version": {
            "full": "1.0.0",
            "short": "1.0",
            "build_id": "123abc",
            "released": "2023-02-01T00:00:00Z"
        }
    },
    "system": {
        "composition": {
            "bases": ["linux"],
            "technology": {
                "core": ["moss", "boulder", "apparmor"],
                "optional": ["selinux"]
            }
        },
        "features": {
            "atomic_updates": {
                "strategy": "none",
                "rollback_support": false
            },
            "boot": {
                "bootloader": "grub",
                "firmware": {
                    "uefi": true,
                    "secure_boot": false,
                    "bios": true
                }
            },
            "filesystem": {
                "default": "ext4",
                "supported": ["ext4"]
            }
        },
        "kernel": {
            "type": "monolithic",
            "name": "linux"
        },
        "platform": {
            "architecture": "x86_64",
            "variant": "generic"
        },
        "update": {
            "strategy": "none",
            "cadence": {
                "type": "fixed",
                "release_schedule": "6 months"
            },
            "approach": "package manager"
        }
    },
    "resources": {
        "websites": {
            "home": {
                "url": "https://example.com",
                "display_name": "Test OS",
                "scope": "home"
            },
            "privacy": {
                "url": "https://example.com/privacy",
                "display_name": "Privacy Policy",
                "scope": "privacy-policy"
            },
            "tos": {
                "url": "https://example.com/terms",
                "display_name": "Terms of Service",
                "scope": "terms-of-service"
            },
            "security": {
                "url": "https://example.com/security",
                "display_name": "Security Policy",
                "scope": "security-policy"
            }
        },
        "social": {},
        "funding": {}
    },
    "security_contact": {
        "email": "security@example.com",
        "pgp_key": "-----BEGIN PGP PUBLIC KEY BLOCK-----\nSample Key\n-----END PGP PUBLIC KEY BLOCK-----",
        "disclosure_policy": "Responsible disclosure within 90 days"
    }
}"#;

    let os_info: OsInfo = serde_json::from_str(json).unwrap();

    // Check security contact
    assert_eq!(
        os_info.security_contact.as_ref().unwrap().email,
        Some("security@example.com".to_string())
    );

    // Check policy links in websites
    let privacy_site = os_info.resources.websites.get("privacy").unwrap();
    assert_eq!(privacy_site.scope, WebsiteScope::PrivacyPolicy);
    assert_eq!(privacy_site.url, "https://example.com/privacy");

    let tos_site = os_info.resources.websites.get("tos").unwrap();
    assert_eq!(tos_site.scope, WebsiteScope::TermsOfService);

    let security_site = os_info.resources.websites.get("security").unwrap();
    assert_eq!(security_site.scope, WebsiteScope::SecurityPolicy);

    // Check technologies in composition (confirming security technologies are included as standard technologies)
    assert!(
        os_info
            .system
            .composition
            .technology
            .core
            .contains(&"apparmor".to_string())
    );
    assert!(
        os_info
            .system
            .composition
            .technology
            .optional
            .contains(&"selinux".to_string())
    );

    // Check conversion to OsRelease
    let release = OsRelease::from(&os_info);
    assert!(release.extra_fields.contains_key("PRIVACY_POLICY_URL"));
    assert_eq!(
        release.extra_fields.get("PRIVACY_POLICY_URL").unwrap(),
        "https://example.com/privacy"
    );
    assert!(release.extra_fields.contains_key("TERMS_OF_SERVICE_URL"));
    assert!(release.extra_fields.contains_key("SECURITY_POLICY_URL"));
    assert!(release.extra_fields.contains_key("SECURITY_CONTACT"));
    assert_eq!(
        release.extra_fields.get("SECURITY_CONTACT").unwrap(),
        "security@example.com"
    );
}

#[test]
fn test_website_scope_serialization() {
    let website = Website {
        url: "https://example.com".to_string(),
        display_name: "Example".to_string(),
        scope: WebsiteScope::Public,
    };

    let serialized = serde_json::to_string(&website).unwrap();
    let deserialized: Website = serde_json::from_str(&serialized).unwrap();

    assert_eq!(website.url, deserialized.url);
    assert!(matches!(deserialized.scope, WebsiteScope::Public));
}

#[test]
fn test_date_parsing() {
    let json = r#"
    {
        "os-info-version": "0.1",
        "start_date": "2023-01-01T00:00:00Z",
        "metadata": {
            "identity": {
                "id": "test-os",
                "name": "Test OS",
                "display": "Test OS",
                "former_identities": [
                    {
                        "id": "old-os",
                        "name": "Old OS",
                        "start_date": "2020-01-01T00:00:00Z",
                        "end_date": "2022-12-31T23:59:59Z"
                    }
                ]
            },
            "maintainers": {
                "core": [
                    {
                        "name": "Test User",
                        "role": "founder",
                        "email": "test@example.com",
                        "start_date": "2023-01-01T00:00:00Z"
                    }
                ]
            },
            "version": {
                "full": "1.0.0",
                "short": "1.0",
                "build_id": "123abc",
                "released": "2023-02-01T00:00:00Z"
            }
        },
        "system": {
            "composition": {
                "bases": [],
                "technology": {
                    "core": [],
                    "optional": []
                }
            },
            "features": {
                "atomic_updates": {
                    "strategy": "none",
                    "rollback_support": false
                },
                "boot": {
                    "bootloader": "grub",
                    "firmware": {
                        "uefi": true,
                        "secure_boot": false,
                        "bios": true
                    }
                },
                "filesystem": {
                    "default": "ext4",
                    "supported": ["ext4"]
                }
            },
            "kernel": {
                "type": "monolithic",
                "name": "linux"
            },
            "platform": {
                "architecture": "x86_64",
                "variant": "generic"
            },
            "update": {
                "strategy": "none",
                "cadence": {
                    "type": "fixed",
                    "release_schedule": "6 months"
                },
                "approach": "package manager"
            }
        },
        "resources": {
            "websites": {
                "home": {
                    "url": "https://example.com",
                    "display_name": "Test OS",
                    "scope": "home"
                }
            },
            "social": {},
            "funding": {}
        }
    }"#;

    let os_info: OsInfo = serde_json::from_str(json).unwrap();

    // Check that dates were properly parsed
    assert_eq!(
        os_info.start_date,
        chrono::DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc)
    );

    let former = &os_info.metadata.identity.former_identities[0];
    assert_eq!(
        former.start_date,
        chrono::DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc)
    );
    assert_eq!(
        former.end_date,
        chrono::DateTime::parse_from_rfc3339("2022-12-31T23:59:59Z")
            .unwrap()
            .with_timezone(&Utc)
    );

    let maintainer = &os_info.metadata.maintainers.get("core").unwrap()[0];
    assert_eq!(
        maintainer.start_date.unwrap(),
        chrono::DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc)
    );

    assert_eq!(
        os_info.metadata.version.released,
        chrono::DateTime::parse_from_rfc3339("2023-02-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc)
    );
}
