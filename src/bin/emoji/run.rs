use crate::document::Document;
use crate::manifest::*;
use crate::render::{process, render};
use emoji_crafter::document::Emoji;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::thread;
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
        let emojis: Vec<_> = document
            .emojis
            .iter()
            .map(|(_, emoji)| emoji.clone())
            .collect();

        let bars = MultiProgress::new();
        let theme_bar = bars.add(ProgressBar::new(project.themes.len() as u64));

        theme_bar.set_style(
            ProgressStyle::default_bar()
                .template(&format!(
                    "{: >10} {{bar:20.cyan/blue}} {{pos:>7}}/{{len:7}} {{msg}}",
                    "themes"
                ))
                .progress_chars("##-"),
        );

        let emoji_bar = bars.add(ProgressBar::new(
            (emojis.len() * 2 * project.themes.len()) as u64,
        ));

        emoji_bar.set_style(
            ProgressStyle::default_bar()
                .template(&format!(
                    "{: >10} {{bar:20.cyan/blue}} {{pos:>7}}/{{len:7}} {{msg}}",
                    "emojis"
                ))
                .progress_chars("##-"),
        );

        let main = thread::spawn(move || {
            theme_bar.set_position(0);
            emoji_bar.set_position(0);

            for theme in &project.themes {
                theme_bar.set_message(theme.name.clone());
                theme_bar.inc(1);

                let mut theme = theme.clone();

                theme.stylesheet = project.path.join(theme.stylesheet.clone());

                let renderable = process(&document.svg, &theme, &emojis);

                for output in &project.outputs {
                    let mut output = output.clone();

                    output.directory = project.path.join(output.directory.clone());

                    render(&renderable, &theme, &output, |emoji: &Emoji| {
                        emoji_bar.set_message(emoji.name().unwrap());
                        emoji_bar.inc(1);
                    });
                }
            }

            emoji_bar.finish();
            theme_bar.finish();
        });

        bars.join().unwrap();
        main.join().unwrap();
    }
}
