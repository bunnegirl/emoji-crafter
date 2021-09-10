use crate::project::*;
use std::{fs::create_dir_all, path::PathBuf, str::FromStr};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Command {
    /// Name of the project
    #[structopt(name = "name")]
    #[structopt(parse(try_from_str = ProjectPath::validate))]
    path: PathBuf,
}

impl Command {
    pub fn run(self) {
        let path = self.path;

        create_dir_all(&path).unwrap();

        let path = path.canonicalize().unwrap();
        let name: String = path.file_stem().unwrap().to_str().unwrap().into();
        let theme_path = path.join("themes");

        create_dir_all(&theme_path).unwrap();

        let emojiset_source = path
            .join("emojiset.svg")
            .strip_prefix(&path)
            .unwrap()
            .to_path_buf();
        let emojiset_stylesheet = path
            .join("emojiset.css")
            .strip_prefix(&path)
            .unwrap()
            .to_path_buf();
        let theme_stylesheet = path
            .join("themes")
            .join("theme.css")
            .strip_prefix(&path)
            .unwrap()
            .to_path_buf();

        touch(&path.join(&emojiset_source)).unwrap();
        touch(&path.join(&emojiset_stylesheet)).unwrap();
        touch(&path.join(&theme_stylesheet)).unwrap();

        let themes = vec![Theme {
            name: name.clone(),
            stylesheet: theme_stylesheet,
        }];
        let emojiset = Emojiset {
            name: name.clone(),
            // version: lenient_semver::parse("0.1").unwrap(),
            source: emojiset_source,
            stylesheet: emojiset_stylesheet,
        };
        let project = Project {
            path: path.clone(),
            emojiset,
            themes,
        };

        let manifest = path.join("emoji.toml");

        std::fs::write(manifest, toml::to_string(&project).unwrap()).unwrap();
    }
}

fn touch(path: &PathBuf) -> std::io::Result<()> {
    match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[derive(Debug)]
enum ProjectPath {
    AlreadyExists,
    InvalidName,
}

impl ProjectPath {
    fn validate(name: &str) -> Result<PathBuf, ProjectPath> {
        use ProjectPath::*;

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

impl std::fmt::Display for ProjectPath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ProjectPath::*;

        match self {
            AlreadyExists => write!(f, "directory already exists, try using `init`"),
            InvalidName => write!(f, "could not create the project directory"),
        }
    }
}
