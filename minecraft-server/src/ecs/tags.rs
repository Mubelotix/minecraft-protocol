use std::{collections::HashSet};

use crate::prelude::Component;


#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Tag {
    Player
}

impl Tag {
    pub fn get_components(&self) -> &[Component] {
        match self {
            Tag::Player => {
                &[Component::Health, Component::Position]
            },
        }
    }

    pub fn get_tags_from_component(component: Component) -> Vec<Tag> {
        match component {
            Component::Health => vec![Tag::Player],
            Component::Position => vec![Tag::Player],
            _ => vec![],
        }
    }

    /// Get all tags that have all the components
    pub fn get_tags_from_components(components: HashSet<Component>) -> HashSet<Tag> {
        let mut tags = HashSet::new();
        for component in components.iter() {
            for tag in Tag::get_tags_from_component(component.clone()) {
                let components_of_tag = tag.get_components();
                if components_of_tag.iter().all(|c| components.contains(c)) {
                    tags.insert(tag);
                }
            }
        }
        tags
    }
}