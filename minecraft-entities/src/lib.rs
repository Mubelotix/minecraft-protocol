#![allow(clippy::derivable_impls)]

mod entity;
pub use entity::*;
mod thrown_item_projectile;
pub use thrown_item_projectile::*;
mod arrow;
pub use arrow::*;
mod boat;
pub use boat::*;
mod living_entity;
pub use living_entity::*;
mod player;
pub use player::*;
mod mobs;
pub use mobs::*;
mod interaction;
pub use interaction::*;
mod animals;
pub use animals::*;
mod display;
pub use display::*;
mod shulker;
pub use shulker::*;
mod monsters;
pub use monsters::*;
mod block;
pub use block::*;
mod particles;
pub use particles::*;
mod fire_entities;
pub use fire_entities::*;
mod item;
pub use item::*;

pub(crate) use minecraft_positions::*;
pub(crate) use minecraft_entities_derive::MinecraftEntity;
pub(crate) use minecraft_protocol::{
    components::{
        entity::Pose,
        slots::{Slot, SlotItem, Hand}
    },
    ids::{items::Item, block_states::BlockWithState},
    nbt::NbtTag,
    packets::UUID
};
use std::{pin::Pin, future::Future};

pub type Eid = u32;

#[allow(dead_code)]
type CallBack<O> = fn(O) -> Pin<Box<dyn Future<Output = ()>>>;
#[allow(dead_code)]
type CallBack1<O, I> = fn(O, I) -> Pin<Box<dyn Future<Output = ()>>>;
#[allow(dead_code)]
type CallBack2<O, I, J> = fn(O, I, J) -> Pin<Box<dyn Future<Output = ()>>>;
#[allow(dead_code)]
type CallBack3<O, I, J> = fn(O, I, J) -> Pin<Box<dyn Future<Output = ()>>>;
#[allow(dead_code)]
type CallBack4<O, I, J> = fn(O, I, J) -> Pin<Box<dyn Future<Output = ()>>>;

pub trait TryAsEntityRef<T> {
    fn try_as_entity_ref(&self) -> Option<&T>;
    fn try_as_entity_mut(&mut self) -> Option<&mut T>;
}


pub trait EntityWorldInterface {
    fn observe_entity(&'static self, eid: Eid, observer: Box<dyn FnOnce(&AnyEntity)>) -> Pin<Box<dyn std::future::Future<Output = ()>>>;
    fn mutate_entity(&'static self, eid: Eid, mutator: Box<dyn FnOnce(&mut AnyEntity)>) -> Pin<Box<dyn std::future::Future<Output = ()>>>;
}

pub struct Handler<T> where AnyEntity: TryAsEntityRef<T> {
    eid: Eid,
    world: &'static dyn EntityWorldInterface,
    entity: std::marker::PhantomData<T>,
}

impl<T: 'static> Handler<T> where AnyEntity: TryAsEntityRef<T> {
    pub fn assume(id: Eid, world: &'static dyn EntityWorldInterface) -> Self {
        Self {
            eid: id,
            world,
            entity: std::marker::PhantomData,
        }
    }

    fn assume_other<V>(self) -> Handler<V> where AnyEntity: TryAsEntityRef<V> {
        Handler {
            eid: self.eid,
            world: self.world,
            entity: std::marker::PhantomData,
        }
    }

    pub async fn observe(&self, observer: impl FnOnce(&T) + 'static) {
        self.world.observe_entity(self.eid, Box::new(move |entity| {
            observer(entity.try_as_entity_ref().unwrap())
        })).await;
    }

    pub async fn mutate(&self, mutator: impl FnOnce(&mut T) + 'static) {
        self.world.mutate_entity(self.eid, Box::new(move |entity| {
            mutator(entity.try_as_entity_mut().unwrap())
        })).await;
    }
}

pub enum AnyEntity {
    Entity(Entity),
    Interaction(Interaction),
    Display(Display),
    BlockDisplay(BlockDisplay),
    ItemDisplay(ItemDisplay),
    TextDisplay(TextDisplay),
    ThrownItemProjectile(ThrownItemProjectile),
    ThrownEgg(ThrownEgg),
    ThrownEnderPearl(ThrownEnderPearl),
    ThrownExperienceBottle(ThrownExperienceBottle),
    ThrownPotion(ThrownPotion),
    Snowball(Snowball),
    AbstractArrow(AbstractArrow),
    Arrow(Arrow),
    SpectralArrow(SpectralArrow),
    ThrownTrident(ThrownTrident),
    Boat(Boat),
    ChestBoat(ChestBoat),
    LivingEntity(LivingEntity),
    Player(Player),
    Mob(Mob),
    AmbientCreature(AmbientCreature),
    Bat(Bat),
    PathfinderMob(PathfinderMob),
    WaterAnimal(WaterAnimal),
    Squid(Squid),
    AgeableMob(AgeableMob),
    Animal(Animal),
    Sniffer(Sniffer),
    AbstractHorse(AbstractHorse),
    Horse(Horse),
    ZombieHorse(ZombieHorse),
    SkeletonHorse(SkeletonHorse),
    Camel(Camel),
    ChestedHorse(ChestedHorse),
    Donkey(Donkey),
    Llama(Llama),
    TraderLlama(TraderLlama),
    Mule(Mule),
    Axolotl(Axolotl),
    Bee(Bee),
    Fox(Fox),
    Frog(Frog),
    Ocelot(Ocelot),
    Panda(Panda),
    Pig(Pig),
    Rabbit(Rabbit),
    Turtle(Turtle),
    PolarBear(PolarBear),
    Chicken(Chicken),
    Cow(Cow),
    Hoglin(Hoglin),
    Mooshroom(Mooshroom),
    Sheep(Sheep),
    Strider(Strider),
    TameableAnimal(TameableAnimal),
    Cat(Cat),
    Wolf(Wolf),
    Parrot(Parrot),
    AbstractVillager(AbstractVillager),
    Villager(Villager),
    WanderingTrader(WanderingTrader),
    AbstractGolem(AbstractGolem),
    IronGolem(IronGolem),
    SnowGolem(SnowGolem),
    Shulker(Shulker),
    Monster(Monster),
    BasePiglin(BasePiglin),
    Piglin(Piglin),
    PiglinBrute(PiglinBrute),
    Blaze(Blaze),
    Creeper(Creeper),
    Endermite(Endermite),
    Giant(Giant),
    Goat(Goat),
    Guardian(Guardian),
    ElderGuardian(ElderGuardian),
    Silverfish(Silverfish),
    Raider(Raider),
    AbstractIllager(AbstractIllager),
    Vindicator(Vindicator),
    Pillager(Pillager),
    SpellcasterIllager(SpellcasterIllager),
    Evoker(Evoker),
    Illusioner(Illusioner),
    Ravager(Ravager),
    Witch(Witch),
    EvokerFangs(EvokerFangs),
    Vex(Vex),
    Skeleton(Skeleton),
    AbstractSkeleton(AbstractSkeleton),
    WitherSkeleton(WitherSkeleton),
    Stray(Stray),
    Spider(Spider),
    Warden(Warden),
    Wither(Wither),
    Zoglin(Zoglin),
    Zombie(Zombie),
    ZombieVillager(ZombieVillager),
    Husk(Husk),
    Drowned(Drowned),
    ZombifiedPiglin(ZombifiedPiglin),
    Enderman(Enderman),
    EnderDragon(EnderDragon),
    Flying(Flying),
    Ghast(Ghast),
    Phantom(Phantom),
    Slime(Slime),
    LlamaSpit(LlamaSpit),
    EyeOfEnder(EyeOfEnder),
    FallingBlock(FallingBlock),
    AreaEffectCloud(AreaEffectCloud),
    FishingHook(FishingHook),
    EndCrystal(EndCrystal),
    DragonFireball(DragonFireball),
    SmallFireball(SmallFireball),
    Fireball(Fireball),
    WitherSkull(WitherSkull),
    FireworkRocket(FireworkRocket),
    ItemFrame(ItemFrame),
    GlowingItemFrame(GlowingItemFrame),
    Painting(Painting),
    ItemEntity(ItemEntity),
    ArmorStand(ArmorStand),
    Dolphin(Dolphin),
    AbstractFish(AbstractFish),
    Cod(Cod),
    PufferFish(PufferFish),
    Salmon(Salmon),
    TropicalFish(TropicalFish),
    Tadpole(Tadpole),
}

impl AnyEntity {
    pub fn as_entity(&self) -> &Entity {
        self.try_as_entity_ref().unwrap()
    }
}
