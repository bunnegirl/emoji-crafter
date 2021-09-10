use crate::project::*;
use std::{path::PathBuf, str::FromStr};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Command {
    /// Name of the project
    #[structopt(default_value = "./")]
    #[structopt(parse(try_from_str = ProjectPath::validate))]
    path: PathBuf,
}

impl Command {
    pub fn run(self) {
        let path = self.path;

        let data = std::fs::read_to_string(&path.join("emoji.toml"))
            .expect("error reading emoji.toml, does the file exist?");
        let mut project = toml::from_str::<Project>(&data)
            .expect("error reading emoji.toml, there may be a syntax error");

        project.path = path;

        // project
    }
}

#[derive(Debug)]
enum ProjectPath {
    NoProject,
}

impl ProjectPath {
    fn validate(name: &str) -> Result<PathBuf, ProjectPath> {
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
