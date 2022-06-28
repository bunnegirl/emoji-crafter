use crate::build::Command as BuildCommand;
use emoji_crafter::prelude::*;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;
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

        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher =
            Watcher::new(tx, Duration::from_secs(2)).expect("error initialising watcher");

        watcher
            .watch(project.path.join("emoji.toml"), RecursiveMode::NonRecursive)
            .expect("error watching emojiset manifest");

        watcher
            .watch(
                project.path.join(&project.emojiset.document),
                RecursiveMode::NonRecursive,
            )
            .expect("error watching emojiset document");

        watcher
            .watch(
                project.path.join(&project.emojiset.stylesheet),
                RecursiveMode::NonRecursive,
            )
            .expect("error watching emojiset stylesheet");

        for theme in &project.themes {
            watcher
                .watch(
                    project.path.join(&theme.stylesheet),
                    RecursiveMode::NonRecursive,
                )
                .expect("error watching emojiset theme");
        }

        for template in &project.templates {
            watcher
                .watch(
                    project.path.join(&template.input),
                    RecursiveMode::NonRecursive,
                )
                .expect("error watching emojiset template");
        }

        let command = BuildCommand { path: project.path };

        build(command.clone());

        loop {
            match rx.recv() {
                Ok(event) => {
                    if let DebouncedEvent::Write(_) = event {
                        build(command.clone());
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    }
}

fn build(command: BuildCommand) {
    clearscreen::clear().unwrap();

    command.run();

    println!("\nWaiting for changes...");
}
