use crate::manifest::Project;
use roxmltree::Node;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Document {
    pub svg: String,
    pub emojis: HashMap<String, Emoji>,
}

impl From<Project> for Document {
    fn from(project: Project) -> Self {
        Self::from(&project)
    }
}

impl From<&Project> for Document {
    fn from(project: &Project) -> Self {
        let svg = std::fs::read_to_string(&project.path.join(&project.emojiset.document))
            .expect("error reading emojiset document, does it exist?");
        let document = roxmltree::Document::parse(&svg)
            .expect("error reading emojiset document, there may be syntax errors");

        let emojis = document
            .descendants()
            .fold(HashMap::new(), |mut emojis, node| {
                if node.has_tag_name("desc") && node.has_children() {
                    if let Some(desc) = node.text() {
                        let emoji: Emoji = toml::from_str(desc).expect(
                            "invalid toml found in description, try using a # comment instead",
                        );
                        let id = get_node_id(&node).expect("missing node id");

                        let emoji = emoji.with_id(&id);

                        // Insert frame into parent emoji
                        if let Emoji::FrameWithId { .. } = emoji {
                            let parent_id =
                                get_parent_id(&node).expect("missing parent id for frame");

                            let parent = emojis
                                .get_mut(&parent_id)
                                .expect("missing parent for frame");

                            if let Emoji::AnimationWithId { frames, .. } = parent {
                                frames.push(emoji);
                            }
                        } else {
                            emojis.insert(id, emoji);
                        }
                    }
                }

                emojis
            });

        Self { svg, emojis }
    }
}

fn get_node_id(node: &Node) -> Option<String> {
    node.ancestors().find_map(|node| {
        if node.has_tag_name("g") {
            if let Some(id) = node.attribute("id") {
                return Some(id.into());
            }
        }

        None
    })
}

fn get_parent_id(node: &Node) -> Option<String> {
    node.ancestors()
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .find_map(|node| {
            if node.has_tag_name("g") {
                if let Some(id) = node.attribute("id") {
                    return Some(id.into());
                }
            }

            None
        })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Emoji {
    #[serde(alias = "animated")]
    Animation {
        name: String,
    },
    #[serde(rename = "animation", alias = "animated")]
    AnimationWithId {
        #[serde(skip_serializing)]
        id: String,
        name: String,
        frames: Vec<Emoji>,
    },
    #[serde(alias = "static")]
    Image {
        name: String,
    },
    #[serde(rename = "image", alias = "static")]
    ImageWithId {
        #[serde(skip_serializing)]
        id: String,
        name: String,
    },
    Frame {
        delay: usize,
        position: usize,
    },
    #[serde(rename = "frame")]
    FrameWithId {
        #[serde(skip_serializing)]
        id: String,
        delay: usize,
        position: usize,
    },
}

impl Emoji {
    pub fn with_id(self, id: &str) -> Self {
        match &self {
            Emoji::Animation { name } => Emoji::AnimationWithId {
                id: id.into(),
                frames: Vec::new(),
                name: name.clone(),
            },
            Emoji::Frame { delay, position } => Emoji::FrameWithId {
                id: id.into(),
                delay: *delay,
                position: *position,
            },
            Emoji::Image { name } => Emoji::ImageWithId {
                id: id.into(),
                name: name.clone(),
            },
            _ => unimplemented!("with id should only be called on raw items parsed from toml"),
        }
    }

    pub fn id(&self) -> Option<String> {
        match self {
            Emoji::AnimationWithId { id, .. } => Some(id.clone()),
            Emoji::FrameWithId { id, .. } => Some(id.clone()),
            Emoji::ImageWithId { id, .. } => Some(id.clone()),
            _ => None,
        }
    }

    pub fn name(&self) -> Option<String> {
        match self {
            Emoji::Animation { name } => Some(name.clone()),
            Emoji::AnimationWithId { name, .. } => Some(name.clone()),
            Emoji::Image { name } => Some(name.clone()),
            Emoji::ImageWithId { name, .. } => Some(name.clone()),
            _ => None,
        }
    }

    pub fn frames(&self) -> Vec<Emoji> {
        match self {
            Emoji::AnimationWithId { frames, .. } => frames.clone(),
            _ => Vec::new(),
        }
    }
}
