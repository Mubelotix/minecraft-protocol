use std::{pin::Pin, future::Future, sync::{Mutex, Arc}};

use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Cow {
    pub animal: Animal,
}

#[derive(Default)]
#[inherit(Cow, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]

pub struct Mooshroom {
    pub cow: Cow,
    pub variant: u8, // In the doc it is a string 
}

// Function that returns a pinned boxed future
type CallBack<O, I> = fn(O, I) -> Pin<Box<dyn Future<Output = ()>>>;

pub struct EntityMethods {
    pub on_jump: CallBack<Entity, ()>,
}

trait EntityExt: Sized + EntityDescendant + Into<Entity> {
    fn methods() -> EntityMethods;

    fn on_jump(self) -> Pin<Box<dyn Future<Output = ()>>> {
        (Self::methods().on_jump)(self.into(), ())
    }
}

pub struct AnimalMethods {
    pub on_hit: CallBack<Animal, f32>,
    pub on_dies: CallBack<Animal, ()>,
}

trait AnimalExt: Sized + AnimalDescendant + Into<Animal> {
    fn methods() -> AnimalMethods;

    fn on_hit(self, damage: f32) -> Pin<Box<dyn Future<Output = ()>>> {
        (Self::methods().on_hit)(self.into(), damage)
    }

    fn on_dies(self) -> Pin<Box<dyn Future<Output = ()>>> {
        (Self::methods().on_dies)(self.into(), ())
    }
}

impl AnimalExt for Animal {
    fn methods() -> AnimalMethods {
        AnimalMethods {
            on_hit: |animal, damage| Box::pin(async {
                println!("Animal was hit");
            }),
            on_dies: |animal, ()| Box::pin(async {
                println!("Animal died");
            }),
        }
    }
}

impl EntityExt for Entity {
    fn methods() -> EntityMethods {
        EntityMethods {
            on_jump: |entity, ()| Box::pin(async {
                println!("Entity jumped");
            }),
        }
    }
}

impl From<Animal> for Entity {
    fn from(val: Animal) -> Self {
        val.ageable_mob.pathfinder_mob.mob.living_entity.entity
    }
}

impl EntityExt for Animal {
    fn methods() -> EntityMethods {
        EntityMethods {
            on_jump: |entity, ()| Box::pin(async {
                println!("Animal jumped");
            }),
        }
    }
}

impl From<Cow> for Animal {
    fn from(val: Cow) -> Self {
        val.animal
    }
}

impl From<Cow> for Entity {
    fn from(val: Cow) -> Self {
        val.animal.ageable_mob.pathfinder_mob.mob.living_entity.entity
    }
}

impl AnimalExt for Cow {
    fn methods() -> AnimalMethods {
        AnimalMethods {
            on_hit: |animal, damage| Box::pin(async {
                println!("Cow was hit");
            }),
            ..<Animal as AnimalExt>::methods()
        }
    }
}

impl EntityExt for Cow {
    fn methods() -> EntityMethods {
        EntityMethods {
            ..<Entity as EntityExt>::methods()
        }
    }
}

#[tokio::test]
async fn test() {
    let cow = Cow::default();
    cow.on_hit(1.0).await;
    let cow = Cow::default();
    cow.on_dies().await;
    let cow = Cow::default();
    cow.on_jump().await;
}
