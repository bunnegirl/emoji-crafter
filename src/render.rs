use crate::document::Emoji;
use crate::manifest::{Output, Theme};
use rayon::prelude::*;
use resvg::trim_transparency;
use std::fs::{create_dir_all, File};
use tiny_skia::{IntRect, Pixmap};
use usvg::{NodeExt, Tree};
use webp_animation::prelude::*;

#[derive(Clone, Debug)]
pub enum RenderableEmoji {
    Image {
        emoji: Emoji,
        pixmap: Pixmap,
    },
    Animation {
        emoji: Emoji,
        width: usize,
        height: usize,
        frames: Vec<(usize, usize, Pixmap)>,
    },
}

/// Apply the theme to emoji and return renderable emoji
pub fn process(svg: &str, theme: &Theme, emojis: &Vec<Emoji>) -> Vec<RenderableEmoji> {
    // Replace emojiset stylesheet with theme stylesheet:
    let mut svg = svg.to_string();
    let path = theme.stylesheet.canonicalize().unwrap();
    let css = std::fs::read_to_string(path).unwrap();

    svg = svg.replace(
        "<style>@import url(emojiset.css);</style>",
        &format!("<style>{}</style>", css),
    );

    let data = svg.as_bytes();

    emojis
        .par_iter()
        .map(|emoji| process_emoji(&emoji, data))
        .collect()
}

fn process_emoji(emoji: &Emoji, data: &[u8]) -> RenderableEmoji {
    let mut opt = usvg::Options::default();

    opt.keep_named_groups = true;

    let rtree = usvg::Tree::from_data(data, &opt.to_ref()).unwrap();

    match emoji {
        Emoji::ImageWithId { .. } => process_image(emoji, rtree),
        Emoji::AnimationWithId { .. } => process_animation(emoji, rtree),
        _ => unreachable!("should not be able to reach this"),
    }
}

fn process_image(emoji: &Emoji, rtree: Tree) -> RenderableEmoji {
    let node = rtree.node_by_id(&emoji.id().unwrap()).unwrap();
    let bbox = node.calculate_bbox().unwrap();
    let mut pixmap = Pixmap::new(bbox.width() as u32, bbox.height() as u32).unwrap();

    resvg::render_node(&rtree, &node, usvg::FitTo::Size(326, 326), pixmap.as_mut()).unwrap();

    RenderableEmoji::Image {
        emoji: emoji.clone(),
        pixmap,
    }
}

fn process_animation(emoji: &Emoji, rtree: Tree) -> RenderableEmoji {
    let mut width: f64 = 0.0;
    let mut height: f64 = 0.0;

    let frames: Vec<_> = emoji
        .frames()
        .iter()
        .map(|frame| {
            if let Emoji::FrameWithId {
                id,
                delay,
                position,
            } = &frame
            {
                let node = rtree.node_by_id(&id).unwrap();
                let bbox = node.calculate_bbox().unwrap();
                let mut pixmap = Pixmap::new(bbox.width() as u32, bbox.height() as u32).unwrap();

                if bbox.width() > width {
                    width = bbox.width();
                }

                if bbox.height() > height {
                    height = bbox.height();
                }

                resvg::render_node(&rtree, &node, usvg::FitTo::Size(326, 326), pixmap.as_mut())
                    .unwrap();

                (*position, *delay, pixmap)
            } else {
                unreachable!("not a frame");
            }
        })
        .collect();

    RenderableEmoji::Animation {
        emoji: emoji.clone(),
        width: width as usize,
        height: height as usize,
        frames,
    }
}

/// Render emoji and write them to disk
pub fn render(emoji: &Vec<RenderableEmoji>, theme: &Theme, output: &Output) {
    emoji.par_iter().for_each(|emoji| {
        render_emoji(emoji, theme, output);
    });
}

pub fn render_emoji(emoji: &RenderableEmoji, theme: &Theme, output: &Output) {
    match emoji {
        RenderableEmoji::Image { emoji, pixmap } => {
            render_image(emoji, pixmap, theme, output);
        }
        RenderableEmoji::Animation {
            emoji,
            width,
            height,
            frames,
        } => {
            render_animation(emoji, *width, *height, frames, theme, output);
        }
    }
}

pub fn render_image(emoji: &Emoji, pixmap: &Pixmap, theme: &Theme, output: &Output) {
    let dir = output.directory.join(&theme.name);
    let path = dir.join(format!("{}{}.png", theme.prefix, emoji.name().unwrap()));

    create_dir_all(&dir).unwrap();

    let pixmap = if output.trim {
        let (_, _, new_pixmap) = trim_transparency(pixmap.clone()).unwrap();

        new_pixmap
    } else {
        pixmap.clone()
    };

    println!("writing image to {}", path.to_str().unwrap());
    pixmap.save_png(path).unwrap();
}

pub fn render_animation(
    emoji: &Emoji,
    width: usize,
    height: usize,
    frames: &Vec<(usize, usize, Pixmap)>,
    theme: &Theme,
    output: &Output,
) {
    let dir = output.directory.join(&theme.name);
    let webp_path = dir.join(format!("{}{}.webp", theme.prefix, emoji.name().unwrap()));
    let gif_path = dir.join(format!("{}{}.gif", theme.prefix, emoji.name().unwrap()));

    create_dir_all(&dir).unwrap();

    let (width, height, trim) = if output.trim {
        // Calculate the actual animation size
        let rect = frames.iter().fold(
            IntRect::from_ltrb(
                (width / 2) as i32,
                (height / 2) as i32,
                (1 + width / 2) as i32,
                (1 + height / 2) as i32,
            )
            .unwrap(),
            |bbox, (_, _, pixmap)| expand_rect(bbox, get_trim_rect(pixmap)),
        );

        (
            rect.width() as usize,
            rect.height() as usize,
            IntRect::from_ltrb(
                rect.left() as i32,
                rect.top() as i32,
                rect.right() as i32,
                rect.bottom() as i32,
            ),
        )
    } else {
        (width, height, None)
    };

    let mut gif = File::create(&gif_path).unwrap();
    let mut gif_encoder = gif::Encoder::new(&mut gif, width as u16, height as u16, &[]).unwrap();
    let mut webp_encoder = Encoder::new((width as u32, height as u32)).unwrap();
    let mut timestamp: usize = 0;

    gif_encoder.set_repeat(gif::Repeat::Infinite).unwrap();

    println!("writing animation to {}", webp_path.to_str().unwrap());
    println!("writing animation to {}", gif_path.to_str().unwrap());

    for (_, delay, pixmap) in frames {
        let mut pixmap = if let Some(trim) = trim {
            pixmap.clone_rect(trim.clone()).unwrap()
        } else {
            pixmap.clone()
        };

        webp_encoder
            .add_frame(pixmap.data(), timestamp as i32)
            .unwrap();

        let mut gif_frame =
            gif::Frame::from_rgba_speed(width as u16, height as u16, pixmap.data_mut(), 30);

        gif_frame.dispose = gif::DisposalMethod::Background;
        gif_frame.delay = (delay / 10) as u16;

        gif_encoder.write_frame(&gif_frame).unwrap();

        timestamp += delay;
    }

    // Save webp:
    let webp = webp_encoder.finalize(timestamp as i32).unwrap();

    std::fs::write(&webp_path, &webp).unwrap();
}

fn expand_rect(a: IntRect, b: IntRect) -> IntRect {
    let left = a.left().min(b.left());
    let top = a.top().min(b.top());
    let right = a.right().max(b.right());
    let bottom = a.bottom().max(b.bottom());

    IntRect::from_ltrb(left, top, right, bottom).unwrap()
}

fn get_trim_rect(pixmap: &Pixmap) -> IntRect {
    let mut x = 0;
    let mut y = 0;
    let width = pixmap.width() as i32;
    let mut min_x = pixmap.width() as i32;
    let mut min_y = pixmap.height() as i32;
    let mut max_x = 0;
    let mut max_y = 0;

    for pixel in pixmap.pixels() {
        if pixel.alpha() != 0 {
            if x < min_x {
                min_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
        }

        x += 1;

        if x == width {
            x = 0;
            y += 1;
        }
    }

    // Expand in all directions by 1px.
    min_x = (min_x - 1).max(0);
    min_y = (min_y - 1).max(0);
    max_x = (max_x + 2).min(pixmap.width() as i32);
    max_y = (max_y + 2).min(pixmap.height() as i32);

    IntRect::from_ltrb(min_x, min_y, max_x, max_y).unwrap()
}
