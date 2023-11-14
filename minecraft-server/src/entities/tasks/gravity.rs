use crate::CollisionShape;

use super::*;

pub async fn gravity_task<T: EntityDescendant>(h: Handler<T>, mut server_msg_rcvr: BroadcastReceiver<ServerMessage>) where AnyEntity: TryAsEntityRef<T> {
    loop {
        let msg = server_msg_rcvr.recv().await.unwrap();

        if !matches!(&msg, &ServerMessage::Tick) {
            continue;
        }

        let Some((mut position, mut velocity, width, height)) = h.observe(|entity| {
            let entity = entity.get_entity();
            (entity.position.clone(), entity.velocity.clone(), 0.6, 1.95)
        }).await else { return; };

        velocity.y -= 9.81/20.0;
        let bounding_box = CollisionShape {
            x1: position.x - width/2.0,
            y1: position.y,
            z1: position.z - width/2.0,
            x2: position.x + width/2.0,
            y2: position.y + height,
            z2: position.z + width/2.0,
        };
        let allowed_velocity = h.world.try_move(&bounding_box, &velocity).await;
        if velocity.x != allowed_velocity.x {
            velocity.x = 0.0;
        }
        if velocity.y != allowed_velocity.y {
            velocity.y = 0.0;
        }
        if velocity.z != allowed_velocity.z {
            velocity.z = 0.0;
        }
        position += allowed_velocity;

        h.mutate(|entity| {
            let entity = entity.get_entity_mut();
            entity.velocity = velocity;
            entity.position = position;
            ((), EntityChanges::position()+EntityChanges::velocity())
        }).await;
    }
}
