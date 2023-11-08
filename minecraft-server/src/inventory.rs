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

    pub fn get_slot(&self, slot: i16) -> Option<&Slot> {
        let idx: usize = slot.try_into().ok()?;
        self.slots.get(idx)
    }

    pub fn get_slot_mut(&mut self, slot: i16) -> Option<&mut Slot> {
        let idx: usize = slot.try_into().ok()?;
        self.slots.get_mut(idx)
    }

    pub fn set_slot(&mut self, slot: i16, item: Slot) {
        let Some(slot) = self.get_slot_mut(slot) else {error!("Tried to set invalid slot {slot}"); return};
        *slot = item;
    }
}
