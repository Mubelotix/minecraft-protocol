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
    Pufferfish(Pufferfish),
    Salmon(Salmon),
    TropicalFish(TropicalFish),
    Tadpole(Tadpole),
}

impl AnyEntity {
    pub fn as_entity(&self) -> &Entity {
        self.try_as_entity_ref().unwrap()
    }

    pub fn as_other<O>(&self) -> Option<&O> where AnyEntity: TryAsEntityRef<O> {
        self.try_as_entity_ref()
    }

    pub fn to_network(&self) -> Option<minecraft_protocol::ids::entities::Entity> {
        use minecraft_protocol::ids::entities::Entity::*;
        match self {
            AnyEntity::Entity(_) => None,
            AnyEntity::Interaction(_) => Some(Interaction),
            AnyEntity::Display(_) => None,
            AnyEntity::BlockDisplay(_) => Some(BlockDisplay),
            AnyEntity::ItemDisplay(_) => Some(ItemDisplay),
            AnyEntity::TextDisplay(_) => Some(TextDisplay),
            AnyEntity::ThrownItemProjectile(_) => None,
            AnyEntity::ThrownEgg(_) => Some(Egg),
            AnyEntity::ThrownEnderPearl(_) => Some(EnderPearl),
            AnyEntity::ThrownExperienceBottle(_) => Some(ExperienceBottle),
            AnyEntity::ThrownPotion(_) => Some(Potion),
            AnyEntity::Snowball(_) => Some(Snowball),
            AnyEntity::AbstractArrow(_) => Some(Arrow), // Default to arrow
            AnyEntity::Arrow(_) => Some(Arrow),
            AnyEntity::SpectralArrow(_) => Some(SpectralArrow),
            AnyEntity::ThrownTrident(_) => Some(Trident),
            AnyEntity::Boat(_) => Some(Boat),
            AnyEntity::ChestBoat(_) => Some(ChestBoat),
            AnyEntity::LivingEntity(_) => None,
            AnyEntity::Player(_) => Some(Player),
            AnyEntity::Mob(_) => None,
            AnyEntity::AmbientCreature(_) => None,
            AnyEntity::Bat(_) => Some(Bat),
            AnyEntity::PathfinderMob(_) => None,
            AnyEntity::WaterAnimal(_) => None,
            AnyEntity::Squid(_) => Some(Squid),
            AnyEntity::AgeableMob(_) => None,
            AnyEntity::Animal(_) => None,
            AnyEntity::Sniffer(_) => Some(Sniffer),
            AnyEntity::AbstractHorse(_) => None,
            AnyEntity::Horse(_) => Some(Horse),
            AnyEntity::ZombieHorse(_) => Some(ZombieHorse),
            AnyEntity::SkeletonHorse(_) => Some(SkeletonHorse),
            AnyEntity::Camel(_) => Some(Camel),
            AnyEntity::ChestedHorse(_) => None,
            AnyEntity::Donkey(_) => Some(Donkey),
            AnyEntity::Llama(_) => Some(Llama),
            AnyEntity::TraderLlama(_) => Some(TraderLlama),
            AnyEntity::Mule(_) => Some(Mule),
            AnyEntity::Axolotl(_) => Some(Axolotl),
            AnyEntity::Bee(_) => Some(Bee),
            AnyEntity::Fox(_) => Some(Fox),
            AnyEntity::Frog(_) => Some(Frog),
            AnyEntity::Ocelot(_) => Some(Ocelot),
            AnyEntity::Panda(_) => Some(Panda),
            AnyEntity::Pig(_) => Some(Pig),
            AnyEntity::Rabbit(_) => Some(Rabbit),
            AnyEntity::Turtle(_) => Some(Turtle),
            AnyEntity::PolarBear(_) => Some(PolarBear),
            AnyEntity::Chicken(_) => Some(Chicken),
            AnyEntity::Cow(_) => Some(Cow),
            AnyEntity::Hoglin(_) => Some(Hoglin),
            AnyEntity::Mooshroom(_) => Some(Mooshroom),
            AnyEntity::Sheep(_) => Some(Sheep),
            AnyEntity::Strider(_) => Some(Strider),
            AnyEntity::TameableAnimal(_) => None,
            AnyEntity::Cat(_) => Some(Cat),
            AnyEntity::Wolf(_) => Some(Wolf),
            AnyEntity::Parrot(_) => Some(Parrot),
            AnyEntity::AbstractVillager(_) => None,
            AnyEntity::Villager(_) => Some(Villager),
            AnyEntity::WanderingTrader(_) => Some(WanderingTrader),
            AnyEntity::AbstractGolem(_) => None,
            AnyEntity::IronGolem(_) => Some(IronGolem),
            AnyEntity::SnowGolem(_) => Some(SnowGolem),
            AnyEntity::Shulker(_) => Some(Shulker),
            AnyEntity::Monster(_) => None,
            AnyEntity::BasePiglin(_) => None,
            AnyEntity::Piglin(_) => Some(Piglin),
            AnyEntity::PiglinBrute(_) => Some(PiglinBrute),
            AnyEntity::Blaze(_) => Some(Blaze),
            AnyEntity::Creeper(_) => Some(Creeper),
            AnyEntity::Endermite(_) => Some(Endermite),
            AnyEntity::Giant(_) => Some(Giant),
            AnyEntity::Goat(_) => Some(Goat),
            AnyEntity::Guardian(_) => Some(Guardian),
            AnyEntity::ElderGuardian(_) => Some(ElderGuardian),
            AnyEntity::Silverfish(_) => Some(Silverfish),
            AnyEntity::Raider(_) => None,
            AnyEntity::AbstractIllager(_) => None,
            AnyEntity::Vindicator(_) => Some(Vindicator),
            AnyEntity::Pillager(_) => Some(Pillager),
            AnyEntity::SpellcasterIllager(_) => None,
            AnyEntity::Evoker(_) => Some(Evoker),
            AnyEntity::Illusioner(_) => Some(Illusioner),
            AnyEntity::Ravager(_) => Some(Ravager),
            AnyEntity::Witch(_) => Some(Witch),
            AnyEntity::EvokerFangs(_) => Some(EvokerFangs),
            AnyEntity::Vex(_) => Some(Vex),
            AnyEntity::Skeleton(_) => Some(Skeleton),
            AnyEntity::AbstractSkeleton(_) => None,
            AnyEntity::WitherSkeleton(_) => Some(WitherSkeleton),
            AnyEntity::Stray(_) => Some(Stray),
            AnyEntity::Spider(_) => Some(Spider),
            AnyEntity::Warden(_) => Some(Warden),
            AnyEntity::Wither(_) => Some(Wither),
            AnyEntity::Zoglin(_) => Some(Zoglin),
            AnyEntity::Zombie(_) => Some(Zombie),
            AnyEntity::ZombieVillager(_) => Some(ZombieVillager),
            AnyEntity::Husk(_) => Some(Husk),
            AnyEntity::Drowned(_) => Some(Drowned),
            AnyEntity::ZombifiedPiglin(_) => Some(ZombifiedPiglin),
            AnyEntity::Enderman(_) => Some(Enderman),
            AnyEntity::EnderDragon(_) => Some(EnderDragon),
            AnyEntity::Flying(_) => None,
            AnyEntity::Ghast(_) => Some(Ghast),
            AnyEntity::Phantom(_) => Some(Phantom),
            AnyEntity::Slime(_) => Some(Slime),
            AnyEntity::LlamaSpit(_) => Some(LlamaSpit),
            AnyEntity::EyeOfEnder(_) => Some(EyeOfEnder),
            AnyEntity::FallingBlock(_) => Some(FallingBlock),
            AnyEntity::AreaEffectCloud(_) => Some(AreaEffectCloud),
            AnyEntity::FishingHook(_) => Some(FishingBobber),
            AnyEntity::EndCrystal(_) => Some(EndCrystal),
            AnyEntity::DragonFireball(_) => Some(DragonFireball),
            AnyEntity::SmallFireball(_) => Some(SmallFireball),
            AnyEntity::Fireball(_) => Some(Fireball),
            AnyEntity::WitherSkull(_) => Some(WitherSkull),
            AnyEntity::FireworkRocket(_) => Some(FireworkRocket),
            AnyEntity::ItemFrame(_) => Some(ItemFrame),
            AnyEntity::GlowingItemFrame(_) => Some(GlowItemFrame),
            AnyEntity::Painting(_) => Some(Painting),
            AnyEntity::ItemEntity(_) => Some(Item),
            AnyEntity::ArmorStand(_) => Some(ArmorStand),
            AnyEntity::Dolphin(_) => Some(Dolphin),
            AnyEntity::AbstractFish(_) => None,
            AnyEntity::Cod(_) => Some(Cod),
            AnyEntity::Pufferfish(_) => Some(Pufferfish),
            AnyEntity::Salmon(_) => Some(Salmon),
            AnyEntity::TropicalFish(_) => Some(TropicalFish),
            AnyEntity::Tadpole(_) => Some(Tadpole),
        }
    }
}
