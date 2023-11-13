use crate::prelude::*;

pub struct PlayerInventory {
    slots: [Slot; 46],
}

impl PlayerInventory {
    pub fn new() -> PlayerInventory {
        const EMPTY_SLOT: Slot = Slot {item: None};
        PlayerInventory {
            slots: [EMPTY_SLOT; 46],
        }
    }

    pub fn get_slot(&self, slot: usize) -> Option<&Slot> {
        self.slots.get(slot)
    }

    pub fn get_slot_mut(&mut self, slot: usize) -> Option<&mut Slot> {
        self.slots.get_mut(slot)
    }

    pub fn set_slot(&mut self, slot: usize, item: Slot) {
        let Some(slot) = self.get_slot_mut(slot) else {error!("Tried to set invalid slot {slot}"); return};
        *slot = item;
    }
}
