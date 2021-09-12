use emoji_crafter::prelude::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::Serialize;
use std::path::PathBuf;
use std::thread;
use structopt::StructOpt;

#[derive(Serialize)]
struct TemplateContext {
    newline: String,
    emojiset: Emojiset,
    emojis: Vec<TemplateEmoji>,
    themes: Vec<Theme>,
    outputs: Vec<Output>,
}

#[derive(Serialize)]
struct TemplateEmoji {
    id: String,
    name: String,
    is_animation: bool,
    is_image: bool,
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
            let emojis = emojis.clone();
            let template_bar = bars.add(ProgressBar::new(project.templates.len() as u64));

            template_bar.set_style(
                ProgressStyle::default_bar()
                    .template(&format!(
                        "{: >10} {{bar:20.green/dim}} {{pos:>7}}/{{len:7}} {{msg}}",
                        "templates"
                    ))
                    .progress_chars(&bar_characters),
            );

            threads.push(thread::spawn(move || {
                let renderable = template_renderer::process(&project, &emojis);

                template_renderer::render(
                    &renderable,
                    &project.templates,
                    |template: &Template| {
                        template_bar.set_message(format!(
                            "{}",
                            template.output.file_name().unwrap().to_str().unwrap()
                        ));
                        template_bar.inc(1);
                    },
                );

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

                let renderable = emoji_renderer::process(&document.svg, &theme, &emojis);

                for output in &project.outputs {
                    let mut output = output.clone();

                    output.directory = project.path.join(output.directory.clone());

                    emoji_renderer::render(&renderable, &theme, &output, |emoji: &Emoji| {
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
