//! M0 path ownership only. No PDX parsing or engine compatibility claims.
use std::{
    io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct SourcePaths {
    pub game_read_only: PathBuf,
    pub mod_authored: PathBuf,
}

impl SourcePaths {
    pub fn inspect(game: &Path, mod_root: &Path) -> io::Result<Self> {
        let game = game.canonicalize()?;
        let mod_root = mod_root.canonicalize()?;
        if !game.is_dir() || !mod_root.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "source roots must be directories",
            ));
        }
        if game.starts_with(&mod_root) || mod_root.starts_with(&game) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "game and mod roots overlap",
            ));
        }
        if !game.join("common/defines/00_defines.txt").is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "not a Victoria 3 game data root",
            ));
        }
        if !mod_root.join(".metadata/metadata.json").is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "mod metadata missing",
            ));
        }
        Ok(Self {
            game_read_only: game,
            mod_authored: mod_root,
        })
    }
}
