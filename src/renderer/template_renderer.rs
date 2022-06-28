use crate::document::Emoji;
use crate::manifest::*;
use rayon::prelude::*;
use serde::Serialize;
use std::path::PathBuf;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
pub struct Renderable {
    path: PathBuf,
    newline: String,
    emojiset: Emojiset,
    emojis: Vec<RenderableEmoji>,
    themes: Vec<Theme>,
    outputs: Vec<Output>,
}

#[derive(Serialize)]
pub struct RenderableEmoji {
    id: String,
    name: String,
    is_animation: bool,
    is_image: bool,
}

pub trait OnProgress<'a>: Fn(&'a Template) {}

impl<'a, T> OnProgress<'a> for T where T: Fn(&'a Template) {}

pub fn process(project: &Project, emojis: &Vec<Emoji>) -> Renderable {
    let mut renderable_emoji: Vec<RenderableEmoji> = emojis
        .par_iter()
        .filter_map(|emoji| match emoji {
            Emoji::Animation { id, name, .. } => Some(RenderableEmoji {
                id: id.clone(),
                name: name.clone(),
                is_animation: true,
                is_image: false,
            }),
            Emoji::Image { id, name, .. } => Some(RenderableEmoji {
                id: id.clone(),
                name: name.clone(),
                is_animation: false,
                is_image: true,
            }),
            _ => None,
        })
        .collect();

    renderable_emoji.par_sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

    Renderable {
        path: project.path.clone(),
        newline: "\n".into(),
        emojiset: project.emojiset.clone(),
        emojis: renderable_emoji,
        themes: project.themes.clone(),
        outputs: project.outputs.clone(),
    }
}

pub fn render<'a, F>(context: &Renderable, templates: &'a Vec<Template>, on_progress: F)
where
    F: OnProgress<'a> + Sync + Send,
{
    templates.par_iter().for_each(|template| {
        let mut renderer = TinyTemplate::new();

        let input_path = context.path.join(template.input.clone());
        let output_path = context.path.join(template.output.clone());

        let input = std::fs::read_to_string(&input_path).unwrap();

        renderer.add_template("current", &input).unwrap();

        match renderer.render("current", &context) {
            Ok(output) => {
                std::fs::write(&output_path, output).unwrap();
            }
            Err(error) => {
                println!("{}", error);

                std::process::exit(1);
            }
        }

        on_progress(template);
    });
}
