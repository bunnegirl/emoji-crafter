use crate::manifest::*;
use std::{fs::create_dir_all, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Command {
    /// Name of the project
    #[structopt(name = "name")]
    #[structopt(parse(try_from_str = NewProjectPath::validate))]
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

        let emojiset_document = path
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

        touch(&path.join(&emojiset_document)).unwrap();
        touch(&path.join(&emojiset_stylesheet)).unwrap();
        touch(&path.join(&theme_stylesheet)).unwrap();

        let outputs = vec![
            Output {
                trim: false,
                directory: path.join("original"),
            },
            Output {
                trim: true,
                directory: path.join("trimmed"),
            },
        ];
        let themes = vec![Theme {
            name: name.clone(),
            stylesheet: theme_stylesheet,
        }];
        let emojiset = Emojiset {
            name: name.clone(),
            document: emojiset_document,
            stylesheet: emojiset_stylesheet,
        };
        let project = Project {
            path: path.clone(),
            emojiset,
            outputs,
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
