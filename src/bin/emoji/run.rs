use crate::document::Document;
use crate::manifest::*;
use crate::render::{process, render};
use emoji_crafter::document::Emoji;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::Serialize;
use std::path::PathBuf;
use std::thread;
use structopt::StructOpt;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct TemplateContext {
    emojiset: Emojiset,
    emojis: Vec<Emoji>,
    themes: Vec<Theme>,
    outputs: Vec<Output>,
}

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
        let bar_characters = "▓▒░";
        let mut threads = Vec::new();

        if !project.templates.is_empty() {
            let project = project.clone();
            let template_bar = bars.add(ProgressBar::new(project.templates.len() as u64));
            let context = TemplateContext {
                emojiset: project.emojiset.clone(),
                emojis: emojis.clone(),
                themes: project.themes.clone(),
                outputs: project.outputs.clone(),
            };

            template_bar.set_style(
                ProgressStyle::default_bar()
                    .template(&format!(
                        "{: >10} {{bar:20.green/dim}} {{pos:>7}}/{{len:7}} {{msg}}",
                        "templates"
                    ))
                    .progress_chars(&bar_characters),
            );

            threads.push(thread::spawn(move || {
                for template in &project.templates {
                    let mut template = template.clone();
                    let mut renderer = TinyTemplate::new();

                    template.input = project.path.join(template.input.clone());
                    template.output = project.path.join(template.output.clone());

                    let input = std::fs::read_to_string(&template.input).unwrap();

                    renderer.add_template("current", &input).unwrap();

                    template_bar.set_message(format!(
                        "{}",
                        template.output.file_name().unwrap().to_str().unwrap()
                    ));
                    template_bar.inc(1);

                    match renderer.render("current", &context) {
                        Ok(output) => {
                            std::fs::write(&template.output, output).unwrap();
                        }
                        Err(error) => {
                            println!("{}", error);

                            std::process::exit(1);
                        }
                    }
                }

                template_bar.finish_with_message("done");
            }));
        }

        let theme_bar = bars.add(ProgressBar::new(project.themes.len() as u64));

        theme_bar.set_style(
            ProgressStyle::default_bar()
                .template(&format!(
                    "{: >10} {{bar:20.green/dim}} {{pos:>7}}/{{len:7}} {{msg}}",
                    "themes"
                ))
                .progress_chars(&bar_characters),
        );

        let emoji_bar = bars.add(ProgressBar::new(
            (emojis.len() * 2 * project.themes.len()) as u64,
        ));

        emoji_bar.set_style(
            ProgressStyle::default_bar()
                .template(&format!(
                    "{: >10} {{bar:20.green/dim}} {{pos:>7}}/{{len:7}} {{msg}}",
                    "emojis"
                ))
                .progress_chars(&bar_characters),
        );

        threads.push(thread::spawn(move || {
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

            emoji_bar.finish_with_message("done");
            theme_bar.finish_with_message("done");
        }));

        bars.join().unwrap();

        for thread in threads {
            let _ = thread.join();
        }
    }
}
