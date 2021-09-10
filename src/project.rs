use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    #[serde(skip)]
    pub path: PathBuf,
    pub emojiset: Emojiset,
    pub themes: Vec<Theme>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Emojiset {
    pub name: String,
    // #[serde(skip)]
    // pub version: Version,
    pub source: PathBuf,
    pub stylesheet: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Theme {
    pub name: String,
    pub stylesheet: PathBuf,
}
