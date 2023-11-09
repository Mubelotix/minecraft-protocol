use minecraft_entities_derive::*;

#[MinecraftEntity(
    parents {  },
    inheritable,
    defines {
        on_moved(self, from: Position, to: Position);
        on_spawned(self);
    }
)]
pub struct Entity {

}

#[MinecraftEntity(
    parents { Entity },
    inheritable,
    defines {
        Entity.on_spawned(self);
        on_hit(self, damage: usize);
        on_jump(self);
    }
)]
pub struct Animal {
    entity: Entity,
}

#[MinecraftEntity(
    parents { Animal, Entity },
    defines {
        Entity.on_spawned(self);
        Animal.on_hit(self);
        on_milked(self);
    }
)]
pub struct Cow {
    animal: Animal,
}



fn main() {

}
