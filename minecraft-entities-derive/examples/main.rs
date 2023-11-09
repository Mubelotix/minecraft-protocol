use minecraft_entities_derive::*;

struct Test {
    test: fn(u8, u8) -> usize,
    test2: fn(u8, u8) -> usize,
}

const BOO: &Test = &Test {
    test: |a, b| a as usize + b as usize,
    test2: |a, b| a as usize + b as usize,
};

const BOO2: &Test = {
    let mut t1 = BOO;
    t1
};

use std::{pin::Pin, future::Future, sync::{Mutex, Arc}};
type CallBack<O> = fn(O) -> Pin<Box<dyn Future<Output = ()>>>;
type CallBack1<O, I> = fn(O, I) -> Pin<Box<dyn Future<Output = ()>>>;
type CallBack2<O, I, J> = fn(O, I, J) -> Pin<Box<dyn Future<Output = ()>>>;
type UUID = u128;

pub struct Handler<T> {
    uuid: UUID,
    world: Arc<Mutex<()>>,
    entity: std::marker::PhantomData<T>,
}

impl<T> Handler<T> {
    fn assume(uuid: UUID, world: Arc<Mutex<()>>) -> Self {
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


#[MinecraftEntity(
    parents {  },
    inheritable,
    defines {
        on_moved(self, from: f32, to: f32);
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
        Animal.on_hit(self, damage: usize);
        on_milked(self);
    }
)]
pub struct Cow {
    animal: Animal,
}



fn main() {

}
