use crate::manifest::Project;
use indexmap::IndexMap;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Document {
    #[serde(skip)]
    pub svg: String,
    #[serde(with = "indexmap::serde_seq")]
    pub emojis: IndexMap<String, Emoji>,
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

        let mut emojis = document
            .descendants()
            .fold(IndexMap::new(), |mut emojis, node| {
                if node.has_tag_name("desc") && node.has_children() {
                    if let Some(desc) = node.text() {
                        let emoji: Emoji = toml::from_str(desc).expect(
                            "invalid toml found in description, try using a # comment instead",
                        );
                        let id = get_node_id(&node).expect("missing node id");

                        let emoji = emoji.init(&id);

                        // Insert frame into parent emoji
                        if let Emoji::Frame { .. } = emoji {
                            let parent_id =
                                get_parent_id(&node).expect("missing parent id for frame");

                            let parent = emojis
                                .get_mut(&parent_id)
                                .expect("missing parent for frame");

                            if let Emoji::Animation { frames, .. } = parent {
                                frames.push(emoji);
                            }
                        } else {
                            emojis.insert(id, emoji);
                        }
                    }
                }

                emojis
            });

        // Make sure animation frames are sorted by position
        emojis.iter_mut().for_each(|(_, emoji)| {
            if let Emoji::Animation { frames, .. } = emoji {
                frames.sort_by(|a, b| {
                    let a = a.position().unwrap();
                    let b = b.position().unwrap();

                    a.cmp(&b)
                });
            }
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
        #[serde(skip)]
        id: String,
        name: String,
        #[serde(skip_deserializing)]
        frames: Vec<Emoji>,
    },
    #[serde(alias = "static")]
    Image {
        #[serde(skip)]
        id: String,
        name: String,
    },
    Frame {
        #[serde(skip_deserializing)]
        id: String,
        delay: usize,
        #[serde(skip_serializing)]
        position: usize,
    },
}

impl Emoji {
    pub fn is_animation(&self) -> bool {
        if let Emoji::Animation { .. } = self {
            return true;
        }

        false
    }

    pub fn init(self, id: &str) -> Self {
        match &self {
            Emoji::Animation { name, frames, .. } => Emoji::Animation {
                id: id.into(),
                name: name.clone(),
                frames: frames.clone(),
            },
            Emoji::Frame {
                delay, position, ..
            } => Emoji::Frame {
                id: id.into(),
                delay: *delay,
                position: *position,
            },
            Emoji::Image { name, .. } => Emoji::Image {
                id: id.into(),
                name: name.clone(),
            },
        }
    }

    pub fn id(&self) -> Option<String> {
        match self {
            Emoji::Animation { id, .. } | Emoji::Frame { id, .. } | Emoji::Image { id, .. } => {
                Some(id.clone())
            }
        }
    }

    pub fn name(&self) -> Option<String> {
        match self {
            Emoji::Animation { name, .. } => Some(name.clone()),
            Emoji::Image { name, .. } => Some(name.clone()),
            _ => None,
        }
    }

    pub fn frames(&self) -> Vec<Emoji> {
        match self {
            Emoji::Animation { frames, .. } => frames.clone(),
            _ => Vec::new(),
        }
    }

    pub fn position(&self) -> Option<usize> {
        match self {
            Emoji::Frame { position, .. } => Some(*position),
            _ => None,
        }
    }
}
