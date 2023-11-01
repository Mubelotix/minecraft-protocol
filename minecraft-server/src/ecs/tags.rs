use std::{collections::HashSet};

use crate::prelude::Components;


#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Tag {
    Player
}

impl Tag {
    pub fn get_components(&self) -> &[Components] {
        match self {
            Tag::Player => {
                &[Components::Health, Components::Position]
            },
        }
    }

    pub fn get_tags_from_component(component: Components) -> Option<Tag> {
        match component {
            Components::Health => Some(Tag::Player),
            Components::Position => Some(Tag::Player),
            _ => None,
        }
    }

    /// Get all tags that have all the components
    pub fn get_tags_from_components(components: HashSet<Components>) -> HashSet<Tag> {
        let mut tags = HashSet::new();
        for component in components.iter() {
            if let Some(tag) = Tag::get_tags_from_component(component.clone()) {
                let components_of_tag = tag.get_components();
                if components_of_tag.iter().all(|c| components.contains(c)) {
                    tags.insert(tag);
                }
            }
        }
        tags
    }
}