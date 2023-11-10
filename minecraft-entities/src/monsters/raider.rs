use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Raider {
    pub monster: Monster,
    pub is_celebrating: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Witch {
    pub raider: Raider,
    pub is_drinking_potion: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct AbstractIllager {
    pub raider: Raider,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractIllager, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Vindicator {
    pub abstract_illager: AbstractIllager,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractIllager, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Pillager {
    pub abstract_illager: AbstractIllager,
    pub is_charging: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { AbstractIllager, Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct SpellcasterIllager {
    pub abstract_illager:  AbstractIllager,
    pub spell: u8,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { SpellcasterIllager, AbstractIllager, Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Illusioner {
    pub spellcaster_illager:  SpellcasterIllager,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { SpellcasterIllager, AbstractIllager, Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Ravager {
    pub spellcaster_illager:  SpellcasterIllager,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { SpellcasterIllager, AbstractIllager, Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Evoker {
    pub spellcaster_illager:  SpellcasterIllager,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { Entity },
)]
pub struct EvokerFangs {
    pub entity: Entity,
}
