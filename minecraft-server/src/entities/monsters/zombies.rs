use super::*;

#[derive(Default, Clone)]
#[MinecraftEntity(
    inheritable,
    ancestors { Monster, PathfinderMob, Mob, LivingEntity, Entity },
    descendants { ZombieVillager, Husk, Drowned, ZombifiedPiglin },
)]
pub struct Zombie {
    pub monster: Monster,
    pub is_baby: bool,
    pub unused: isize,
    pub is_becoming_drowned: bool,
}

// TODO: Attributes should be stored in a nicer way
// https://minecraft.wiki/w/Attribute
const ZOMBIE_BASE_FOLLOW_RANGE: f64 = 35.0;
const ZOMBIE_BASE_MOVEMENT_SPEED: f64 = 0.23;

pub struct ZombieTask {
    newton_task: NewtonTask,
    target: Option<Eid>,
}

impl ZombieTask {
    pub async fn init(zombie: &Zombie) -> Option<ZombieTask> {
        let anyentity: AnyEntity = zombie.to_owned().into();
        let Some(newton_task) = NewtonTask::init(&anyentity).await else { return None; };
        Some(ZombieTask {
            newton_task,
            target: None
        })
    }

    /// Sets the target to the closest player in range.
    /// 
    /// Returns the position of the zombie and the position of the target as an optimization, just so that we don't have to get them again.
    async fn acquire_target(&mut self, h: &Handler<Zombie>) -> Option<(Position, Position)> {
        // Get the range of chunks to search
        let self_position = h.observe(|e| e.get_entity().position.clone()).await?;
        let mut lowest = self_position.clone();
        lowest.x -= ZOMBIE_BASE_FOLLOW_RANGE.floor();
        lowest.z -= ZOMBIE_BASE_FOLLOW_RANGE.floor();
        let mut highest = self_position.clone();
        highest.x += ZOMBIE_BASE_FOLLOW_RANGE.ceil();
        highest.z += ZOMBIE_BASE_FOLLOW_RANGE.ceil();
        let lowest_chunk = lowest.chunk_column();
        let highest_chunk = highest.chunk_column();

        // List all players in area
        let mut player_positions = HashMap::new();
        for cx in lowest_chunk.cx..=highest_chunk.cx {
            for cz in lowest_chunk.cz..=highest_chunk.cz {
                let chunk_position = ChunkColumnPosition { cx, cz };
                h.world.observe_entities(chunk_position, |entity, eid| -> Option<()> {
                    TryAsEntityRef::<Player>::try_as_entity_ref(entity).map(|player| {
                        player_positions.insert(eid, player.get_entity().position.clone());
                    });
                    None
                }).await;
            }
        }

        // Return if no players are found
        if player_positions.is_empty() {
            return None;
        }

        // Get their distances
        let mut player_distances = Vec::with_capacity(player_positions.len());
        for (eid, position) in &player_positions {
            player_distances.push((*eid, position.distance(&self_position)));
        }
        player_distances.sort_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap());

        // Get the closest player that's in range
        let (target_eid, target_dist) = player_distances[0];
        if target_dist > ZOMBIE_BASE_FOLLOW_RANGE as f64 {
            return None;
        }
        self.target = Some(target_eid);

        // TODO: ensure there is a line of sight

        player_positions.remove(&target_eid).map(|target_position| (self_position, target_position))
    }

    /// Returns the position of the target if any.
    async fn get_target_position(&self, h: &Handler<Zombie>) -> Option<Position> {
        let target_eid = self.target?;
        h.world.observe_entity(target_eid, |entity| {
            TryAsEntityRef::<Entity>::try_as_entity_ref(entity).map(|player| {
                player.position.clone()
            })
        }).await.flatten()
    }

    /// Returns the position of the zombie.
    async fn get_self_position(&self, h: &Handler<Zombie>) -> Option<Position> {
        h.observe(|e| e.get_entity().position.clone()).await
    }

    /// Returns the movement towards the target that can be applied without colliding with the world.
    async fn get_movement(&self, h: &Handler<Zombie>, self_position: &Position, target_position: &Position) -> Translation {
        // Create a movement vector
        let mut translation = Translation {
            x: target_position.x - self_position.x,
            y: target_position.y - self_position.y,
            z: target_position.z - self_position.z,
        };
        if translation.norm() > ZOMBIE_BASE_MOVEMENT_SPEED {
            translation.set_norm(ZOMBIE_BASE_MOVEMENT_SPEED);
        }

        // Create a collision shape
        let collision_shape = CollisionShape {
            x1: self_position.x - 0.5,
            y1: self_position.y,
            z1: self_position.z - 0.5,
            x2: self_position.x + 0.5,
            y2: self_position.y + 1.95,
            z2: self_position.z + 0.5,
        };

        // Restrict the movement considering world collisions
        h.world.try_move(&collision_shape, &translation).await
    }

    pub async fn tick(&mut self, h: Handler<Zombie>, entity_change_set: &EntityChangeSet) {
        // Acquire target if none
        let mut positions = None;
        if self.target.is_none() {
            positions = self.acquire_target(&h).await;
        }

        // Get target position if not already acquired
        if positions.is_none() {
            let target_position = self.get_target_position(&h).await;
            let self_position = self.get_self_position(&h).await;
            positions = match (target_position, self_position) {
                (Some(target_position), Some(self_position)) => Some((self_position, target_position)),
                _ => return,
            };
        }

        // Get the movement to apply
        if let Some((self_position, target_position)) = positions {
            let movement = self.get_movement(&h, &self_position, &target_position).await;
            h.mutate(|e| {
                e.get_entity_mut().position += movement;
            }).await;
        }

        self.newton_task.tick(h.into(), entity_change_set).await;
    }
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Zombie, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct ZombieVillager {
    pub zombie: Zombie,
    pub is_converting: bool,
    pub villager_data: Vec<u8>,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Zombie, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Husk {
    pub zombie: Zombie,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Zombie, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Drowned {
    pub zombie: Zombie,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Zombie, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct ZombifiedPiglin {
    pub zombie: Zombie,
}
