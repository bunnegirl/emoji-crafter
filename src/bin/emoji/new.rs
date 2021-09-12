use emoji_crafter::prelude::*;
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

        let emojiset_document = PathBuf::from("emojiset.svg");
        let emojiset_stylesheet = PathBuf::from("emojiset.css");
        let theme_stylesheet = PathBuf::from("themes").join(format!("{}.css", &name));

        touch(&path.join(&emojiset_document)).unwrap();
        touch(&path.join(&emojiset_stylesheet)).unwrap();
        touch(&path.join(&theme_stylesheet)).unwrap();

        let outputs = vec![
            Output {
                trim: false,
                directory: "original".into(),
            },
            Output {
                trim: true,
                directory: "trimmed".into(),
            },
        ];
        let templates = vec![];
        let themes = vec![Theme {
            name: name.clone(),
            prefix: "".into(),
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
            templates,
            themes,
            outputs,
        };

        let manifest = path.join("emoji.toml");

        std::fs::write(&manifest, toml::to_string(&project).unwrap()).unwrap();
        std::fs::write(
            path.join(&project.emojiset.document),
            include_str!("../../../tpl/emojiset.svg"),
        )
        .unwrap();
        std::fs::write(
            path.join(&project.emojiset.stylesheet),
            include_str!("../../../tpl/emojiset.css"),
        )
        .unwrap();
        std::fs::write(
            path.join(&project.themes[0].stylesheet),
            include_str!("../../../tpl/theme.css"),
        )
        .unwrap();

        println!(
            "Created new emojiset project in {}",
            manifest
                .strip_prefix(std::env::current_dir().unwrap())
                .unwrap()
                .to_str()
                .unwrap()
        );
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
