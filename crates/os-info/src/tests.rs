use super::*;
use crate::load_os_info_from_path;
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
    assert_eq!(former.id, "serpent-os");
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
    };

    let serialized = serde_json::to_string(&maintainer).unwrap();
    let deserialized: Maintainer = serde_json::from_str(&serialized).unwrap();

    assert_eq!(maintainer.name, deserialized.name);
    assert!(matches!(deserialized.role, MaintainerRole::Founder));
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
