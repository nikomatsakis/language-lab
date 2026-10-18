use std::{fs, path::Path};

#[test]
fn symposium_installation_and_skill_match_root_package() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let cargo: toml::Value =
        toml::from_str(&fs::read_to_string(root.join("Cargo.toml")).unwrap()).unwrap();
    let plugin: toml::Value =
        toml::from_str(&fs::read_to_string(root.join("SYMPOSIUM.toml")).unwrap()).unwrap();
    let package = &cargo["package"];
    let command = plugin["subcommand"]["lab"]["command"].as_str().unwrap();
    let install = plugin["installations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|i| i["name"].as_str() == Some(command))
        .unwrap();
    assert_eq!(install["source"].as_str(), Some("cargo"));
    assert_eq!(install["crate"], package["name"]);
    assert_eq!(install["version"], package["version"]);
    assert_eq!(install["git"], package["repository"]);
    assert_eq!(install["executable"].as_str(), Some("language-lab"));
    let groups = plugin["skills"].as_array().unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0]["source"]["path"].as_str(), Some("skills"));
    assert!(root.join("skills/language-lab/SKILL.md").is_file());
}
