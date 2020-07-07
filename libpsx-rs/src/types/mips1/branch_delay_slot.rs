#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct BranchDelaySlot {
    target: Option<u32>,
    slots: u32,
}

impl BranchDelaySlot {
    pub(crate) fn new() -> BranchDelaySlot {
        BranchDelaySlot {
            target: None,
            slots: 0,
        }
    }

    pub(crate) fn advance(&mut self) -> Option<u32> {
        if self.target.is_none() {
            return None;
        }

        if self.slots == 0 {
            let target = self.target.unwrap();
            self.target = None;
            return Some(target);
        }

        self.slots -= 1;
        None
    }

    pub(crate) fn advance_all(&mut self) -> Option<u32> {
        if let Some(t) = self.target {
            self.slots = 0;
            self.target = None;
            Some(t)
        } else {
            None
        }
    }

    pub(crate) fn back(&mut self) {
        if self.target.is_some() {
            self.slots += 1;
        }
    }

    pub(crate) fn set(&mut self, target: u32, slots: u32) {
        debug_assert!(slots > 0);
        self.target = Some(target);
        self.slots = slots;
    }

    pub(crate) fn branching(&self) -> bool {
        self.target.is_some()
    }

    pub(crate) fn target_or_null(&self) -> u32 {
        self.target.unwrap_or(0x0)
    }
}
