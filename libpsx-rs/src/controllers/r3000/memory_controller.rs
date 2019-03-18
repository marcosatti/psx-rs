pub fn translate_address(va: u32) -> u32 {
    match va {
        // kuseg.
        // The PSX doesn't have a TLB, but it also uses a special mapping that
        // differs from the standard MIPS documentation.
        0x0000_0000...0x7FFF_FFFF => {
            va
        },
        // kseg0.
        0x8000_0000...0x9FFF_FFFF => {
            va - 0x8000_0000
        },
        // kseg1.
        0xA000_0000...0xBFFF_FFFF => {
            va - 0xA000_0000
        },
        // kseg2.
        0xC000_0000...0xFFFD_FFFF => {
            unimplemented!("Address translation reached kseg2 - unimplemented")
        },
        // Cache control i/o ports (PSX specific).
        0xFFFE_0000...0xFFFF_FFFF => {
            va
        },
    }
}
