pub use super::*;

mod newton;
pub use newton::*;

//pub trait EntityTask where AnyEntity: TryAsEntityRef<<Self as tasks::EntityTask>::InnerEntity> {
//    type InnerEntity;
//
//    async fn init(h: Handler<Self::InnerEntity>) -> Option<Box<Self>>;
//    async fn tick(&mut self, h: Handler<Self::InnerEntity>);
//}

pub enum EntityTask {
    Zombie(ZombieTask),
}

impl EntityTask {
    pub async fn init(entity: &AnyEntity) -> Option<EntityTask> {
        match entity {
            AnyEntity::Zombie(zombie) => ZombieTask::init(zombie).await.map(EntityTask::Zombie),
            _ => None,
        }
    }

    pub async fn tick(&mut self, h: Handler<Entity>, entity_change_set: &EntityChangeSet) {
        match self {
            EntityTask::Zombie(zombie_task) => zombie_task.tick(h.assume_other(), entity_change_set).await,
        }
    }
}
