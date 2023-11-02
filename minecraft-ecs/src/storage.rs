use std::{collections::{HashMap, HashSet}, sync::RwLock};

use crate::{entity::Eid, component::{Component, ComponentId}};


pub struct ComponentStorage {
    table: RwLock<HashMap<ComponentId, HashSet<Eid>>>,
    components: RwLock<HashMap<(ComponentId, Eid), Box<dyn Component>>>
}

impl ComponentStorage {
    pub async fn attach_component<T: Component>(&self, id: Eid, component: T) -> Option<()> {
        let component_id: ComponentId = component.get_component_id();

        let mut table = self.table.write().ok()?;
        table.entry(component_id.clone()).or_default().insert(id);
        drop(table);

        let mut components = self.components.write().ok()?;
        components.insert((component_id, id), Box::new(component));

        Some(())
    }

    pub async fn get_component<T: Component + Clone + 'static>(&self, id: Eid, component_id: ComponentId) -> Option<T> {
        let components = self.components.read().ok()?;
        let component = components.get(&(component_id, id))?;
                
        let component = component.as_any().downcast_ref::<T>()?.clone();
        Some(component)
    }
    
    
    pub async fn update_component<T: Component>(&self, id: Eid, component: T) -> Option<()> {
        let component_id = component.get_component_id();
        let mut components = self.components.write().ok()?;
        components.insert((component_id, id), Box::new(component));
        Some(())
    }

    pub async fn remove_component(&self, id: Eid, component_id: ComponentId) -> Option<()> {
        let mut table = self.table.write().ok()?;
        table.entry(component_id.clone()).or_default().remove(&id);
        drop(table);

        let mut components = self.components.write().ok()?;
        components.remove(&(component_id, id));
        Some(())
    }

    pub async fn get_entities_with(&self, component_id: ComponentId) -> HashSet<Eid> {
        let table = self.table.read().ok().unwrap();
        table.get(&component_id).cloned().unwrap_or_default()
    }
}

impl Default for ComponentStorage {
    fn default() -> Self {
        Self {
            table: RwLock::new(HashMap::new()),
            components: RwLock::new(HashMap::new()),
        }
    }
}