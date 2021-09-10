use emoji_crafter::*;
use structopt::StructOpt;

/// Create, manage and export emojis
#[derive(StructOpt, Debug)]
#[structopt(name = "emoji")]
enum Opt {
    /// Create a new project
    New(new::Command),
    /// Run the current project
    Run(run::Command),
}

fn main() {
    match Opt::from_args() {
        Opt::New(cmd) => cmd.run(),
        Opt::Run(cmd) => cmd.run(),
    }
}

// use rayon::prelude::*;
// use resvg::trim_transparency;
// use serde::{Deserialize, Serialize};
// use std::fs::{create_dir_all, File};
// use std::path::{Path, PathBuf};
// use tiny_skia::Pixmap;
// use webp_animation::prelude::*;
// use xmltree::{Element, XMLNode};

// #[derive(Clone, Debug, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// enum Emoji {
//     Static {
//         name: String,
//         id: Option<String>,
//     },
//     Animated {
//         name: String,
//         id: Option<String>,
//         #[serde(default)]
//         frames: Vec<Frame>,
//     },
// }

// impl Emoji {
//     fn is_animated(&self) -> bool {
//         if let &Emoji::Animated { .. } = self {
//             return true;
//         }

//         false
//     }

//     fn with_frames(self, frames: Vec<Frame>) -> Emoji {
//         if let Emoji::Animated { name, id, .. } = self {
//             Emoji::Animated { name, id, frames }
//         } else {
//             self
//         }
//     }

//     fn with_id(self, id: Option<String>) -> Emoji {
//         match self {
//             Emoji::Static { name, .. } => Emoji::Static { name, id },
//             Emoji::Animated { name, frames, .. } => Emoji::Animated { name, frames, id },
//         }
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// struct Frame {
//     id: Option<String>,
//     position: usize,
//     delay: i32,
// }

// impl Frame {
//     fn with_id(self, id: Option<String>) -> Frame {
//         Frame { id, ..self }
//     }
// }

// fn load_emojis(emoji_node: &XMLNode) -> Option<Emoji> {
//     if let XMLNode::Element(emoji_node) = emoji_node {
//         if emoji_node.name == "g" {
//             if let Some(_) = emoji_node.get_child("desc") {
//                 let desc = emoji_node.get_child("desc").unwrap().get_text().unwrap();
//                 let mut emoji: Emoji = toml::from_str(&desc).unwrap();

//                 if let Some(id) = emoji_node.attributes.get("id") {
//                     emoji = emoji.with_id(Some(id.clone()));
//                 }

//                 if emoji.is_animated() {
//                     let mut frames: Vec<_> =
//                         emoji_node.children.iter().filter_map(load_frames).collect();

//                     frames.sort_by(|a, b| a.position.cmp(&b.position));

//                     emoji = emoji.with_frames(frames);
//                 }

//                 return Some(emoji);
//             }
//         }
//     }

//     None
// }

// fn load_frames(node: &XMLNode) -> Option<Frame> {
//     if let XMLNode::Element(node) = node {
//         if node.name == "g" {
//             if let Some(_) = node.get_child("desc") {
//                 let desc = node.get_child("desc").unwrap().get_text().unwrap();
//                 let mut frame: Frame = toml::from_str(&desc).unwrap();

//                 if let Some(id) = node.attributes.get("id") {
//                     frame = frame.with_id(Some(id.clone()));
//                 }

//                 return Some(frame);
//             }
//         }
//     }

//     None
// }

// fn setup_directories() -> (PathBuf, PathBuf) {
//     let out_dir = Path::new("./").canonicalize().unwrap();
//     let squared_dir = out_dir.join("squared");
//     let trimmed_dir = out_dir.join("trimmed");

//     create_dir_all(&squared_dir).unwrap();
//     create_dir_all(&trimmed_dir).unwrap();

//     (squared_dir, trimmed_dir)
// }

// #[derive(Debug)]
// struct TrimRect {
//     left: i32,
//     top: i32,
//     right: i32,
//     bottom: i32,
// }

// impl TrimRect {
//     fn new(width: i32, height: i32) -> Self {
//         let left = width / 2;
//         let top = height / 2;
//         let right = left;
//         let bottom = top;

//         TrimRect {
//             left,
//             top,
//             right,
//             bottom,
//         }
//     }

//     fn expand(&mut self, pixmap: &tiny_skia::Pixmap) {
//         let mut x = 0;
//         let mut y = 0;
//         let width = pixmap.width() as i32;
//         let mut min_x = pixmap.width() as i32;
//         let mut min_y = pixmap.height() as i32;
//         let mut max_x = 0;
//         let mut max_y = 0;

//         for pixel in pixmap.pixels() {
//             if pixel.alpha() != 0 {
//                 if x < min_x {
//                     min_x = x;
//                 }
//                 if y < min_y {
//                     min_y = y;
//                 }
//                 if x > max_x {
//                     max_x = x;
//                 }
//                 if y > max_y {
//                     max_y = y;
//                 }
//             }

//             x += 1;
//             if x == width {
//                 x = 0;
//                 y += 1;
//             }
//         }

//         // Expand in all directions by 1px.
//         min_x = (min_x - 1).max(0);
//         min_y = (min_y - 1).max(0);
//         max_x = (max_x + 2).min(pixmap.width() as i32);
//         max_y = (max_y + 2).min(pixmap.height() as i32);

//         self.left = self.left.min(min_x);
//         self.top = self.top.min(min_y);
//         self.right = self.right.max(max_x);
//         self.bottom = self.bottom.max(max_y);
//     }
// }

// fn main() {
//     // let (squared_dir, trimmed_dir) = setup_directories();

//     // let data = include_str!("emoji.svg");
//     // let mut doc = Element::parse(data.as_bytes()).unwrap();
//     // let emojis: Vec<Emoji> = doc.children.iter().filter_map(load_emojis).collect();

//     // // Replace stylesheet:
//     // let mut stylesheet = doc.get_mut_child("style").unwrap();

//     // stylesheet.children = vec![XMLNode::Text(include_str!("../themes/bunne.css").into())];

//     // let mut buf = Vec::new();
//     // doc.write(&mut buf).unwrap();

//     // let data = String::from_utf8(buf).unwrap();
//     // let mut opt = usvg::Options::default();

//     // opt.keep_named_groups = true;

//     // emojis.par_iter().for_each(|emoji| {
//     //     let rtree = usvg::Tree::from_data(data.as_bytes(), &opt.to_ref()).unwrap();

//     //     match emoji.clone() {
//     //         Emoji::Static { name, id } => {
//     //             let id = id.unwrap();
//     //             let node = rtree.node_by_id(&id).unwrap();
//     //             let mut pixmap = Pixmap::new(326, 326).unwrap();

//     //             resvg::render_node(&rtree, &node, usvg::FitTo::Size(326, 326), pixmap.as_mut())
//     //                 .unwrap();

//     //             // Save squared version:
//     //             pixmap
//     //                 .save_png(squared_dir.join(format!("bunne{}.png", name)))
//     //                 .unwrap();

//     //             // Save trimmed version:
//     //             let (_, _, pixmap) = trim_transparency(pixmap).unwrap();

//     //             pixmap
//     //                 .save_png(trimmed_dir.join(format!("bunne{}.png", name)))
//     //                 .unwrap();
//     //         }
//     //         Emoji::Animated { name, frames, .. } => {
//     //             let mut image_rect = TrimRect::new(326, 326);

//     //             let frames: Vec<_> = frames
//     //                 .iter()
//     //                 .map(|frame| {
//     //                     let id = frame.id.as_ref().unwrap();
//     //                     let node = rtree.node_by_id(&id).unwrap();
//     //                     let mut pixmap = Pixmap::new(326, 326).unwrap();

//     //                     resvg::render_node(
//     //                         &rtree,
//     //                         &node,
//     //                         usvg::FitTo::Size(326, 326),
//     //                         pixmap.as_mut(),
//     //                     )
//     //                     .unwrap();

//     //                     image_rect.expand(&pixmap);

//     //                     (frame, pixmap)
//     //                 })
//     //                 .collect();

//     //             let rect = tiny_skia::IntRect::from_ltrb(
//     //                 image_rect.left,
//     //                 image_rect.top,
//     //                 image_rect.right,
//     //                 image_rect.bottom,
//     //             )
//     //             .unwrap();

//     //             let mut squared_encoder = Encoder::new((326, 326)).unwrap();
//     //             let mut trimmed_encoder =
//     //                 Encoder::new((rect.width() as u32, rect.height() as u32)).unwrap();
//     //             let mut timestamp: i32 = 0;
//     //             let mut squared_gif =
//     //                 File::create(squared_dir.join(format!("bunne{}.gif", name))).unwrap();
//     //             let mut squared_gif_encoder =
//     //                 gif::Encoder::new(&mut squared_gif, 326, 326, &[]).unwrap();
//     //             let mut trimmed_gif =
//     //                 File::create(trimmed_dir.join(format!("bunne{}.gif", name))).unwrap();
//     //             let mut trimmed_gif_encoder = gif::Encoder::new(
//     //                 &mut trimmed_gif,
//     //                 rect.width() as u16,
//     //                 rect.height() as u16,
//     //                 &[],
//     //             )
//     //             .unwrap();

//     //             squared_gif_encoder
//     //                 .set_repeat(gif::Repeat::Infinite)
//     //                 .unwrap();

//     //             trimmed_gif_encoder
//     //                 .set_repeat(gif::Repeat::Infinite)
//     //                 .unwrap();

//     //             for (frame, mut squared) in frames {
//     //                 squared_encoder
//     //                     .add_frame(squared.data(), timestamp)
//     //                     .unwrap();

//     //                 let mut gif_frame =
//     //                     gif::Frame::from_rgba_speed(326, 326, squared.data_mut(), 30);

//     //                 gif_frame.dispose = gif::DisposalMethod::Background;
//     //                 gif_frame.delay = (frame.delay / 10) as u16;

//     //                 squared_gif_encoder.write_frame(&gif_frame).unwrap();

//     //                 let mut trimmed = squared.clone_rect(rect.clone()).unwrap();

//     //                 trimmed_encoder
//     //                     .add_frame(trimmed.data(), timestamp)
//     //                     .unwrap();

//     //                 let mut gif_frame = gif::Frame::from_rgba_speed(
//     //                     rect.width() as u16,
//     //                     rect.height() as u16,
//     //                     trimmed.data_mut(),
//     //                     30,
//     //                 );

//     //                 gif_frame.dispose = gif::DisposalMethod::Background;
//     //                 gif_frame.delay = (frame.delay / 10) as u16;

//     //                 trimmed_gif_encoder.write_frame(&gif_frame).unwrap();

//     //                 timestamp += frame.delay;
//     //             }

//     //             // Save squared version:
//     //             let webp_data = squared_encoder.finalize(timestamp).unwrap();
//     //             let webp_file = squared_dir.join(format!("bunne{}.webp", name));

//     //             std::fs::write(&webp_file, &webp_data).unwrap();

//     //             // Save trimmed version:
//     //             let webp_data = trimmed_encoder.finalize(timestamp).unwrap();
//     //             let webp_file = trimmed_dir.join(format!("bunne{}.webp", name));

//     //             std::fs::write(&webp_file, &webp_data).unwrap();
//     //         }
//     //     }
//     // });
// }
