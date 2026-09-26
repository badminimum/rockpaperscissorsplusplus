use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    pub id: String,
    pub pretty_name: String,
    pub beats: Vec<String>,
}

impl Concept {
    pub fn new(id: impl Into<String>, pretty_name: impl Into<String>, beats: &[&str]) -> Self {
        Self {
            id: id.into(),
            pretty_name: pretty_name.into(),
            beats: beats.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn defaults() -> [Concept; 3] {
        let rock = Self::new("rock", "Rock", &["scissors"]);
        let paper = Self::new("paper", "Paper", &["rock"]);
        let scissors = Self::new("scissors", "Scissors", &["paper"]);

        [rock, paper, scissors]
    }
}

impl Hash for Concept {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Concept {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Concept {}

impl Borrow<str> for Concept {
    fn borrow(&self) -> &str {
        &self.id
    }
}
