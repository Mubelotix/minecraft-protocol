#![allow(clippy::derivable_impls)]

mod entity;
pub use entity::*;
mod thrown_item_projectile;
pub use thrown_item_projectile::*;
mod arrow;
pub use arrow::*;
mod boat;
pub use boat::*;
mod chest_boat;
pub use chest_boat::*;
mod living_entity;
pub use living_entity::*;
mod player;
pub use player::*;
mod mob;
pub use mob::*;
mod ambient_creature;
pub use ambient_creature::*;
mod bat;
pub use bat::*;
mod pathfinder_mob;
pub use pathfinder_mob::*;
mod water_animal;
pub use water_animal::*;
mod squid;
pub use squid::*;
mod interaction;
pub use interaction::*;
mod ageable_mob;
pub use ageable_mob::*;
mod animals;
pub use animals::*;
mod display;
pub use display::*;
mod villagers;
pub use villagers::*;
mod golems;
pub use golems::*;
mod shulker;
pub use shulker::*;
mod monsters;
pub use monsters::*;
mod ender_dragon;
pub use ender_dragon::*; 
mod flying;
pub use flying::*;  
mod slime;
pub use slime::*;

pub use minecraft_server_derive::{inherit, inheritable};

pub use crate::prelude::*;
pub use minecraft_protocol::{
    components::{
        entity::Pose,
        slots::{Slot, SlotItem, Hand}
    },
    ids::{items::Item, block_states::BlockWithState},
    nbt::NbtTag
};

pub enum AnyEntity {
    Entity(Entity),
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
    Axolott(Axolott),
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
}

#[allow(clippy::single_match)]
impl AnyEntity {
    pub fn as_entity(&self) -> &Entity {
        match self {
            AnyEntity::Entity(entity) => entity,
            AnyEntity::Display(display) => display.get_entity(),
            AnyEntity::BlockDisplay(block_display) => block_display.get_entity(),
            AnyEntity::ItemDisplay(item_display) => item_display.get_entity(),
            AnyEntity::TextDisplay(text_display) => text_display.get_entity(),
            AnyEntity::ThrownItemProjectile(throw_item_projectile) => throw_item_projectile.get_entity(),
            AnyEntity::ThrownEgg(throw_egg) => throw_egg.get_entity(),
            AnyEntity::ThrownEnderPearl(throw_ender_pearl) => throw_ender_pearl.get_entity(),
            AnyEntity::ThrownExperienceBottle(throw_experience_bottle) => throw_experience_bottle.get_entity(),
            AnyEntity::ThrownPotion(throw_potion) => throw_potion.get_entity(),
            AnyEntity::Snowball(snowball) => snowball.get_entity(),
            AnyEntity::AbstractArrow(abstract_arrow) => abstract_arrow.get_entity(),
            AnyEntity::Arrow(arrow) => arrow.get_entity(),
            AnyEntity::SpectralArrow(spectral_arrow) => spectral_arrow.get_entity(),
            AnyEntity::ThrownTrident(throw_trident) => throw_trident.get_entity(),
            AnyEntity::Boat(boat) => boat.get_entity(),
            AnyEntity::ChestBoat(chest_boat) => chest_boat.get_entity(),
            AnyEntity::LivingEntity(living_entity) => living_entity.get_entity(),
            AnyEntity::Player(player) => player.get_entity(),
            AnyEntity::Mob(mob) => mob.get_entity(),
            AnyEntity::AmbientCreature(ambient_creature) => ambient_creature.get_entity(),
            AnyEntity::Bat(bat) => bat.get_entity(),
            AnyEntity::PathfinderMob(pathfinder_mob) => pathfinder_mob.get_entity(),
            AnyEntity::WaterAnimal(water_animal) => water_animal.get_entity(),
            AnyEntity::Squid(squid) => squid.get_entity(),
            AnyEntity::AgeableMob(ageable_mob) => ageable_mob.get_entity(),
            AnyEntity::Animal(animal) => animal.get_entity(),
            AnyEntity::Sniffer(sniffer) => sniffer.get_entity(),
            AnyEntity::AbstractHorse(abstract_horse) => abstract_horse.get_entity(),
            AnyEntity::ZombieHorse(zombie_horse) => zombie_horse.get_entity(),
            AnyEntity::Horse(horse) => horse.get_entity(),
            AnyEntity::SkeletonHorse(skeleton_horse) => skeleton_horse.get_entity(),
            AnyEntity::Camel(camel) => camel.get_entity(),
            AnyEntity::ChestedHorse(chested_horse) => chested_horse.get_entity(),
            AnyEntity::Donkey(donkey) => donkey.get_entity(),
            AnyEntity::Llama(llama) => llama.get_entity(),
            AnyEntity::TraderLlama(trader_llama) => trader_llama.get_entity(),
            AnyEntity::Mule(mule) => mule.get_entity(),
            AnyEntity::Axolott(axolotl) => axolotl.get_entity(),
            AnyEntity::Bee(bee) => bee.get_entity(),
            AnyEntity::Fox(fox) => fox.get_entity(),
            AnyEntity::Frog(frog) => frog.get_entity(),
            AnyEntity::Ocelot(ocelot) => ocelot.get_entity(),
            AnyEntity::Panda(panda) => panda.get_entity(),
            AnyEntity::Pig(pig) => pig.get_entity(),
            AnyEntity::Rabbit(rabbit) => rabbit.get_entity(),
            AnyEntity::Turtle(turtle) => turtle.get_entity(),
            AnyEntity::PolarBear(polar_bear) => polar_bear.get_entity(),
            AnyEntity::Chicken(chicken) => chicken.get_entity(),
            AnyEntity::Cow(cow) => cow.get_entity(),
            AnyEntity::Hoglin(hoglin) => hoglin.get_entity(),
            AnyEntity::Mooshroom(mooshroom) => mooshroom.get_entity(),
            AnyEntity::Sheep(sheep) => sheep.get_entity(),
            AnyEntity::Strider(strider) => strider.get_entity(),
            AnyEntity::TameableAnimal(tameable_animal) => tameable_animal.get_entity(),
            AnyEntity::Cat(cat) => cat.get_entity(),
            AnyEntity::Wolf(wolf) => wolf.get_entity(),
            AnyEntity::Parrot(parrot) => parrot.get_entity(),
            AnyEntity::AbstractVillager(abstract_villager) => abstract_villager.get_entity(),
            AnyEntity::Villager(villager) => villager.get_entity(),
            AnyEntity::WanderingTrader(wandering_trader) => wandering_trader.get_entity(),
            AnyEntity::AbstractGolem(abstract_golem) => abstract_golem.get_entity(),
            AnyEntity::IronGolem(iron_golem) => iron_golem.get_entity(),
            AnyEntity::SnowGolem(snow_golem) => snow_golem.get_entity(),
            AnyEntity::Shulker(shulker) => shulker.get_entity(),
            AnyEntity::Monster(monster) => monster.get_entity(),
            AnyEntity::BasePiglin(base_piglin) => base_piglin.get_entity(),
            AnyEntity::Piglin(piglin) => piglin.get_entity(),
            AnyEntity::PiglinBrute(piglin_brute) => piglin_brute.get_entity(),
            AnyEntity::Blaze(blaze) => blaze.get_entity(),
            AnyEntity::Creeper(creeper) => creeper.get_entity(),
            AnyEntity::Endermite(endermite) => endermite.get_entity(),
            AnyEntity::Giant(giant) => giant.get_entity(),
            AnyEntity::Goat(goat) => goat.get_entity(),
            AnyEntity::Guardian(guardian) => guardian.get_entity(),
            AnyEntity::ElderGuardian(elder_guardian) => elder_guardian.get_entity(),
            AnyEntity::Silverfish(silverfish) => silverfish.get_entity(),
            AnyEntity::Raider(raider) => raider.get_entity(),
            AnyEntity::AbstractIllager(abstract_illager) => abstract_illager.get_entity(),
            AnyEntity::Vindicator(vindicator) => vindicator.get_entity(),
            AnyEntity::Pillager(pillager) => pillager.get_entity(),
            AnyEntity::SpellcasterIllager(spellcaster_illager) => spellcaster_illager.get_entity(),
            AnyEntity::Evoker(evoker) => evoker.get_entity(),
            AnyEntity::Illusioner(illusioner) => illusioner.get_entity(),
            AnyEntity::Ravager(ravager) => ravager.get_entity(),
            AnyEntity::Witch(witch) => witch.get_entity(),
            AnyEntity::EvokerFangs(evoker_fangs) => evoker_fangs.get_entity(),
            AnyEntity::Vex(vex) => vex.get_entity(),
            AnyEntity::Skeleton(skeleton) => skeleton.get_entity(),
            AnyEntity::AbstractSkeleton(abstract_skeleton) => abstract_skeleton.get_entity(), 
            AnyEntity::WitherSkeleton(wither_skeleton) => wither_skeleton.get_entity(),
            AnyEntity::Stray(stray) => stray.get_entity(), 
            AnyEntity::Spider(spider) => spider.get_entity(),      
            AnyEntity::Warden(warden) => warden.get_entity(),
            AnyEntity::Wither(wither) => wither.get_entity(),
            AnyEntity::Zoglin(zoglin) => zoglin.get_entity(),
            AnyEntity::Zombie(zombie) => zombie.get_entity(),
            AnyEntity::ZombieVillager(zombie_villager) => zombie_villager.get_entity(),
            AnyEntity::Husk(husk) => husk.get_entity(),
            AnyEntity::Drowned(drowned) => drowned.get_entity(),
            AnyEntity::ZombifiedPiglin(zombified_piglin) => zombified_piglin.get_entity(),
            AnyEntity::Enderman(enderman) => enderman.get_entity(),
            AnyEntity::EnderDragon(ender_dragon) => ender_dragon.get_entity(),
            AnyEntity::Flying(flying) => flying.get_entity(),
            AnyEntity::Ghast(ghast) => ghast.get_entity(),
            AnyEntity::Phantom(phantom) => phantom.get_entity(),
            AnyEntity::Slime(slime) => slime.get_entity(),
            AnyEntity::LlamaSpit(llama_spit) => llama_spit.get_entity(),
            AnyEntity::EyeOfEnder(eye_of_ender) => eye_of_ender.get_entity(),
        }   
    }

    pub fn as_display(&self) -> Option<&Display> {
        match self {
            AnyEntity::Display(display) => Some(display),
            AnyEntity::BlockDisplay(block_display) => Some(&block_display.display),
            AnyEntity::ItemDisplay(item_display) => Some(&item_display.display),
            AnyEntity::TextDisplay(text_display) => Some(&text_display.display),
            _ => None,
        }
    }

    pub fn as_thrown_item_projectile(&self) -> Option<&ThrownItemProjectile> {
        match self {
            AnyEntity::ThrownItemProjectile(throw_item_projectile) => Some(throw_item_projectile),
            AnyEntity::ThrownEgg(throw_egg) => Some(&throw_egg.thrown_item_projectile),
            AnyEntity::ThrownEnderPearl(throw_ender_pearl) => Some(&throw_ender_pearl.thrown_item_projectile),
            AnyEntity::ThrownExperienceBottle(throw_experience_bottle) => Some(&throw_experience_bottle.thrown_item_projectile),
            AnyEntity::ThrownPotion(throw_potion) => Some(&throw_potion.thrown_item_projectile),
            AnyEntity::Snowball(snowball) => Some(&snowball.thrown_item_projectile),
            _ => None,
        }
    }

    pub fn as_abstract_arrow(&self) -> Option<&AbstractArrow> {
        match self {
            AnyEntity::AbstractArrow(abstract_arrow) => Some(abstract_arrow),
            AnyEntity::Arrow(arrow) => Some(&arrow.abstract_arrow),
            AnyEntity::SpectralArrow(spectral_arrow) => Some(&spectral_arrow.abstract_arrow),
            AnyEntity::ThrownTrident(throw_trident) => Some(&throw_trident.abstract_arrow),
            _ => None,
        }
    }

    pub fn as_boat(&self) -> Option<&Boat> {
        match self {
            AnyEntity::Boat(boat) => Some(boat),
            AnyEntity::ChestBoat(chest_boat) => Some(&chest_boat.boat),
            _ => None,
        }
    }

    pub fn as_living_entity(&self) -> Option<&LivingEntity> {
        match self {
            AnyEntity::LivingEntity(living_entity) => return Some(living_entity),
            AnyEntity::Player(player) => return Some(&player.living_entity),
            _ => (),
        };
        if let Some(mob) = self.as_mob() {
            return Some(&mob.living_entity);
        }
        None
    }

    pub fn as_mob(&self) -> Option<&Mob> {
        match self {
            AnyEntity::Mob(mob) => return Some(mob),
            AnyEntity::PathfinderMob(pathfinder_mob) => return Some(&pathfinder_mob.mob),
            _ => (),
        };
        if let Some(ambient_creature) = self.as_ambient_creature() {
            return Some(&ambient_creature.mob);
        }
        None
    }

    pub fn as_ambient_creature(&self) -> Option<&AmbientCreature> {
        match self {
            AnyEntity::AmbientCreature(ambient_creature) => Some(ambient_creature),
            AnyEntity::Bat(bat) => Some(&bat.ambient_creature),
            _ => None,
        }
    }

    pub fn as_pathfinder_mob(&self) -> Option<&PathfinderMob> {
        match self {
            AnyEntity::PathfinderMob(pathfinder_mob) => return Some(pathfinder_mob),
            _ => (),
        }
        if let Some(water_animal) = self.as_water_animal() {
            return Some(&water_animal.pathfinder_mob);
        }
        None
    }

    pub fn as_water_animal(&self) -> Option<&WaterAnimal> {
        match self {
            AnyEntity::WaterAnimal(water_animal) => Some(water_animal),
            AnyEntity::Squid(squid) => Some(&squid.water_animal),
            _ => None,
        }
    }
}
