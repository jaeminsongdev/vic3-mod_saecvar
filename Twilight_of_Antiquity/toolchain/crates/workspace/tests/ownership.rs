use saecvar_workspace::SourcePaths;
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "saecvar-path-test-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(root.join("game/common/defines")).unwrap();
        fs::write(
            root.join("game/common/defines/00_defines.txt"),
            "NGame = {}\n",
        )
        .unwrap();
        fs::create_dir_all(root.join("mod/.metadata")).unwrap();
        fs::write(root.join("mod/.metadata/metadata.json"), "{}").unwrap();
        Self(root)
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn distinct_roots_are_read_without_mutation() {
    let f = Fixture::new();
    let before = fs::read(f.0.join("game/common/defines/00_defines.txt")).unwrap();
    SourcePaths::inspect(&f.0.join("game"), &f.0.join("mod")).unwrap();
    assert_eq!(
        before,
        fs::read(f.0.join("game/common/defines/00_defines.txt")).unwrap()
    );
}

#[test]
fn both_directions_of_containment_are_rejected() {
    let f = Fixture::new();
    assert!(SourcePaths::inspect(&f.0, &f.0.join("mod")).is_err());
    assert!(SourcePaths::inspect(&f.0.join("game"), &f.0).is_err());
    assert!(SourcePaths::inspect(&f.0.join("game"), &f.0.join("game")).is_err());
}

#[test]
fn missing_input_and_wrong_game_root_are_rejected() {
    let f = Fixture::new();
    assert!(SourcePaths::inspect(&f.0.join("missing"), &f.0.join("mod")).is_err());
    assert!(SourcePaths::inspect(&f.0.join("mod"), &f.0.join("game")).is_err());
}

#[test]
fn directory_traversal_cannot_disguise_overlap() {
    let f = Fixture::new();
    assert!(SourcePaths::inspect(&f.0.join("game"), &f.0.join("mod/../game")).is_err());
}
