use crate::document::Document;
use crate::manifest::*;
use std::path::PathBuf;
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

        let _doc = Document::from(&project);

        todo!("implement a custom serialiser for document because toml-rs doesn't know how to handle enums");

        // println!("{:#?}", toml::to_string_pretty(&doc.emojis));
    }
}
