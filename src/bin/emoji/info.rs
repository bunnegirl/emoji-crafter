use emoji_crafter::prelude::*;
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

        println!("{}", toml::to_string_pretty(&document.emojis).unwrap());
    }
}
