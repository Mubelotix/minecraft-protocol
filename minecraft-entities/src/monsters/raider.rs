use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable,
    ancestors { Monster, PathfinderMob, Mob, LivingEntity, Entity },
    descendants { Witch, AbstractIllager... },
)]
pub struct Raider {
    pub monster: Monster,
    pub is_celebrating: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Witch {
    pub raider: Raider,
    pub is_drinking_potion: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    inheritable,
    ancestors { Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity },
    descendants { Vindicator, Pillager, SpellcasterIllager... },
)]
pub struct AbstractIllager {
    pub raider: Raider,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { AbstractIllager, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Vindicator {
    pub abstract_illager: AbstractIllager,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { AbstractIllager, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Pillager {
    pub abstract_illager: AbstractIllager,
    pub is_charging: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    inheritable,
    ancestors { AbstractIllager, Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity },
    descendants { Illusioner, Ravager, Evoker },
)]
pub struct SpellcasterIllager {
    pub abstract_illager:  AbstractIllager,
    pub spell: u8,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { SpellcasterIllager, AbstractIllager, Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Illusioner {
    pub spellcaster_illager:  SpellcasterIllager,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { SpellcasterIllager, AbstractIllager, Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Ravager {
    pub spellcaster_illager:  SpellcasterIllager,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { SpellcasterIllager, AbstractIllager, Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Evoker {
    pub spellcaster_illager:  SpellcasterIllager,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Entity },
)]
pub struct EvokerFangs {
    pub entity: Entity,
}
