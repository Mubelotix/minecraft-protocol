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

impl Handler<Animal> {
    async fn on_hit(self, damage: f32) {
        println!("Animal was hit");
    }

    async fn on_dies(self) {
        println!("Animal died");
    }
}

impl Handler<Cow> {
    async fn on_hit(self, damage: f32) {
        println!("Cow was hit");
    }
}

// Function that returns a pinned boxed future
type CallBack<O> = fn(O) -> Pin<Box<dyn Future<Output = ()>>>;
type CallBack1<O, I> = fn(O, I) -> Pin<Box<dyn Future<Output = ()>>>;

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

// Entity:

pub struct EntityMethods {
    pub on_jump: CallBack<Handler<Entity>>,
}

trait EntityExt: Sized + Into<Handler<Entity>> {
    fn methods() -> EntityMethods;

    fn on_jump(self) -> Pin<Box<dyn Future<Output = ()>>> {
        (Self::methods().on_jump)(self.into())
    }
}

impl EntityExt for Handler<Entity> {
    fn methods() -> EntityMethods {
        EntityMethods {
            on_jump: |entity| Box::pin(async {
                println!("Entity jumped");
            }),
        }
    }
}

/// Animal:
/// 
/// ```
/// #[
///     inheritable,
///     methods(
///         on_hit(self, damage: f32);
///         on_dies(self);
///     )
/// ]
/// struct Animal {
///    pub entity: Entity,
/// }
/// 
/// impl Handler<Animal> {
///     async fn on_hit(self, damage: f32) {
///         println!("Animal was hit");
///     }
/// 
///     async fn on_dies(self) {
///         println!("Animal died");
///     }
/// }
/// ```

pub struct AnimalMethods {
    pub on_hit: CallBack1<Handler<Animal>, f32>,
    pub on_dies: CallBack<Handler<Animal>>,
}

trait AnimalExt: Sized + Into<Handler<Animal>> {
    fn methods() -> AnimalMethods;

    fn on_hit(self, damage: f32) -> Pin<Box<dyn Future<Output = ()>>> {
        (Self::methods().on_hit)(self.into(), damage)
    }

    fn on_dies(self) -> Pin<Box<dyn Future<Output = ()>>> {
        (Self::methods().on_dies)(self.into())
    }
}

impl AnimalExt for Handler<Animal> {
    fn methods() -> AnimalMethods {
        AnimalMethods {
            on_hit: |animal, damage| Box::pin(animal.on_hit(damage)),
            on_dies: |animal| Box::pin(animal.on_dies()),
        }
    }
}

impl From<Handler<Animal>> for Handler<Entity> {
    fn from(val: Handler<Animal>) -> Self {
        val.assume_other()
    }
}

impl EntityExt for Handler<Animal> {
    fn methods() -> EntityMethods {
        EntityMethods {
            ..<Handler<Entity>>::methods()
        }
    }
}

/// Cow:
/// 
/// ```rust
/// #[override(
///     animal.on_hit(self, damage: f32);
/// )]
/// struct Cow {
///     pub animal: Animal,
/// }
/// 
/// impl Handler<Cow> {
///     async fn on_hit(self, damage: f32) {
///         println!("Cow was hit");
///     }
/// }
/// ```

impl From<Handler<Cow>> for Handler<Entity> {
    fn from(val: Handler<Cow>) -> Self {
        val.assume_other()
    }
}

impl EntityExt for Handler<Cow> {
    fn methods() -> EntityMethods {
        EntityMethods {
            ..<Handler<Animal> as EntityExt>::methods()
        }
    }
}

impl From<Handler<Cow>> for Handler<Animal> {
    fn from(val: Handler<Cow>) -> Self {
        val.assume_other()
    }
}

impl AnimalExt for Handler<Cow> {
    fn methods() -> AnimalMethods {
        AnimalMethods {
            on_hit: |animal, damage| Box::pin(animal.assume_other::<Cow>().on_hit(damage)),
            ..<Handler<Animal> as AnimalExt>::methods()
        }
    }
}

#[tokio::test]
async fn test() {
    let cow = Handler::<Cow>::assume(0, Arc::new(Mutex::new(())));
    cow.on_hit(1.0).await;
    let cow = Handler::<Cow>::assume(0, Arc::new(Mutex::new(())));
    cow.on_dies().await;
    let cow = Handler::<Cow>::assume(0, Arc::new(Mutex::new(())));
    cow.on_jump().await;
}
