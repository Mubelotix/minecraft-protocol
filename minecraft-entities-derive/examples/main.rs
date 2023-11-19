use minecraft_entities_derive::*;

use std::{pin::Pin, future::Future, sync::{Mutex, Arc}};
type CallBack<O> = fn(O) -> Pin<Box<dyn Future<Output = ()> + Sync + Send>>;
type CallBack1<O, I> = fn(O, I) -> Pin<Box<dyn Future<Output = ()> + Sync + Send>>;
type CallBack2<O, I, J> = fn(O, I, J) -> Pin<Box<dyn Future<Output = ()> + Sync + Send>>;
type Eid = u32;

trait TryAsEntityRef<T> {
    fn try_as_entity_ref(&self) -> Option<&T>;
    fn try_as_entity_mut(&mut self) -> Option<&mut T>;
}

enum AnyEntity {
    Entity(Entity),
    Animal(Animal),
    Cow(Cow),
}

pub struct Handler<T> {
    uuid: Eid,
    world: Arc<Mutex<()>>,
    entity: std::marker::PhantomData<T>,
}

impl<T> Handler<T> {
    fn assume(uuid: Eid, world: Arc<Mutex<()>>) -> Self {
        Self {
            uuid,
            world,
            entity: std::marker::PhantomData,
        }
    }

    fn assume_other<V>(self) -> Handler<V> {
        Handler {
            uuid: self.uuid,
            world: self.world,
            entity: std::marker::PhantomData,
        }
    }
}

// Entity

#[MinecraftEntity(
    inheritable,
    ancestors {  },
    descendants { Animal... },
    defines {
        on_moved(self, from: f32, to: f32);
        on_spawned(self);
    }
)]
pub struct Entity {

}

impl Handler<Entity> {
    async fn on_moved(self, from: f32, to: f32) {
        println!("Entity moved from {} to {}", from, to);
    }

    async fn on_spawned(self) {
        println!("Entity spawned");
    }
}

// Animal

#[MinecraftEntity(
    inheritable,
    ancestors { Entity },
    descendants { Cow },
    defines {
        Entity.on_spawned(self);
        on_hit(self, damage: usize);
        on_jump(self);
    }
)]
pub struct Animal {
    entity: Entity,
}

impl Handler<Animal> {
    async fn on_hit(self, damage: usize) {
        println!("Animal hit with {} damage", damage);
    }

    async fn on_jump(self) {
        println!("Animal jumped");
    }

    async fn on_spawned(self) {
        println!("Animal spawned");
    }
}

// Cow

#[MinecraftEntity(
    ancestors { Animal, Entity },
    defines {
        Entity.on_spawned(self);
        Animal.on_hit(self, damage: usize);
        on_milked(self);
    }
)]
pub struct Cow {
    animal: Animal,
}

impl Handler<Cow> {
    async fn on_milked(self) {
        println!("Cow milked");
    }

    async fn on_hit(self, damage: usize) {
        println!("Cow hit with {} damage", damage);
    }

    async fn on_spawned(self) {
        println!("Cow spawned");
    }
}

fn main() {
}

#[test]
fn test() {

}
