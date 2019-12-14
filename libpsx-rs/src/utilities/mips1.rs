/// Returns the actual jump target given the current PC address (register).
/// A jump target is 24-bits long, OR'd with the upper 6 bits of the current PC,
/// and the lower 2 bits set to 0 (word aligned).
pub fn pc_calc_jump_target(pc_value: u32, target: u32) -> u32 {
    (pc_value & 0xF000_0000) | (target << 2)
}

/// Pushes the COP0 status register exception and kernel states (like a stack).
/// IEc and KUc are set to 0, other status values get the more recent copy.
pub fn status_push_exception(status_value: u32) -> u32 {
    let ie_ku = status_value & 0x3F;
    let ie_ku_new = (ie_ku << 2) & 0x3F;
    (status_value & !0x3F) | (ie_ku_new & 0x3F)
}

/// Pops the COP0 status register exception and kernel states (like a stack).
/// KUo and IEo are left unchanged while other status values get the more
/// recent copy.
pub fn status_pop_exception(status_value: u32) -> u32 {
    let ie_ku = status_value & 0x3F;
    let ie_ku_new = (ie_ku & 0x30) | (ie_ku >> 2);
    (status_value & !0x3F) | (ie_ku_new & 0x3F)
}
