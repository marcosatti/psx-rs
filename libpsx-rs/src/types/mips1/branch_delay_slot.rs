pub struct BranchDelaySlot {
    target: Option<u32>,
    slots: u32,
}

impl BranchDelaySlot {
    pub fn new() -> BranchDelaySlot {
        BranchDelaySlot {
            target: None,
            slots: 0,
        }
    }

    pub fn advance(&mut self) -> Option<u32> {
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

    pub fn advance_all(&mut self) -> Option<u32> {
        if let Some(t) = self.target {
            self.slots = 0;
            self.target = None;
            Some(t)
        } else {
            None
        }
    }

    pub fn back(&mut self) {
        if self.target.is_some() {
            self.slots += 1;
        }
    }

    pub fn set(&mut self, target: u32, slots: u32) {
        debug_assert!(slots > 0);
        self.target = Some(target);
        self.slots = slots;
    }

    pub fn branching(&self) -> bool {
        self.target.is_some()
    }

    pub fn cancel(&mut self) {
        assert!(self.branching());
        self.target = None;
        self.slots = 0;
    }

    pub fn target_or_null(&self) -> u32 {
        self.target.unwrap_or(0x0)
    }
}
