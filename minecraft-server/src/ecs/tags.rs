use std::collections::HashSet;
use crate::components::*;
use tags_macros::tags;


tags! {
    Player {
        Position,
        Health,
    }
    Enemy {
        Health
    }
}