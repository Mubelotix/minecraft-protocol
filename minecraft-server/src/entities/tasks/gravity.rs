use super::*;

pub async fn gravity_task<T: EntityDescendant>(h: Handler<T>, mut server_msg_rcvr: BroadcastReceiver<ServerMessage>) where AnyEntity: TryAsEntityRef<T> {
    loop {
        let msg = server_msg_rcvr.recv().await.unwrap();

        if !matches!(&msg, &ServerMessage::Tick) {
            continue;
        }

        h.mutate(|entity| {
            let entity = entity.get_entity_mut();
            entity.velocity.y -= 9.81/20.0;
            entity.position += entity.velocity.clone();
            ((), EntityChanges::position()+EntityChanges::velocity())
        }).await;
    }
}
