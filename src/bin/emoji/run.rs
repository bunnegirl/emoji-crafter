use crate::document::Document;
use crate::manifest::*;
use crate::render::{process, render};
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

        let document = Document::from(&project);
        let emojis = document
            .emojis
            .iter()
            .map(|(_, emoji)| emoji.clone())
            .collect();

        for theme in &project.themes {
            let mut theme = theme.clone();

            theme.stylesheet = project.path.join(theme.stylesheet.clone());

            let renderable = process(&document.svg, &theme, &emojis);

            for output in &project.outputs {
                let mut output = output.clone();

                output.directory = project.path.join(output.directory.clone());

                render(&renderable, &theme, &output);
            }
        }
    }
}
