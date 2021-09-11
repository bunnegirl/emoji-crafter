use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Project {
    #[serde(skip)]
    pub path: PathBuf,
    pub emojiset: Emojiset,
    #[serde(alias = "theme")]
    pub themes: Vec<Theme>,
    #[serde(alias = "output")]
    pub outputs: Vec<Output>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Emojiset {
    pub name: String,
    pub document: PathBuf,
    pub stylesheet: PathBuf,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Output {
    pub trim: bool,
    pub directory: PathBuf,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Theme {
    pub name: String,
    pub stylesheet: PathBuf,
}

#[derive(Debug)]
pub enum ProjectPath {
    NoProject,
}

impl ProjectPath {
    pub fn validate(name: &str) -> Result<PathBuf, ProjectPath> {
        use ProjectPath::*;

        match PathBuf::from_str(name) {
            Ok(path) => match path.join("emoji.toml").canonicalize() {
                Ok(_) => Ok(path.canonicalize().unwrap()),
                Err(_) => Err(NoProject),
            },
            Err(_) => Err(NoProject),
        }
    }
}

impl std::fmt::Display for ProjectPath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ProjectPath::*;

        match self {
            NoProject => write!(f, "the current directory is not an emoji project"),
        }
    }
}

#[derive(Debug)]
pub enum NewProjectPath {
    AlreadyExists,
    InvalidName,
}

impl NewProjectPath {
    pub fn validate(name: &str) -> Result<PathBuf, NewProjectPath> {
        use NewProjectPath::*;

        match PathBuf::from_str(name) {
            // We should not be able to canonicalize the path as it should not already exist
            Ok(path) => match path.canonicalize() {
                Ok(_) => Err(AlreadyExists),
                // The path must have a file stem to use as the project name
                Err(_) => match path.file_stem() {
                    Some(_) => Ok(path),
                    None => Err(InvalidName),
                },
            },
            Err(_) => Err(InvalidName),
        }
    }
}

impl std::fmt::Display for NewProjectPath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use NewProjectPath::*;

        match self {
            AlreadyExists => write!(f, "directory already exists, try using `init`"),
            InvalidName => write!(f, "could not create the project directory"),
        }
    }
}
