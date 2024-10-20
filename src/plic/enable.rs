#[repr(C)]
#[doc = "PLIC interrupt enable registers (per-HART)"]
#[doc(alias = "enable")]
pub struct Enable {
    enable_bits: [EnableBits; 5],
}
impl Enable {
    #[doc = "0x00..0x14 - Interrupt source enable bits"]
    #[inline(always)]
    pub const fn enable_bits(&self, n: usize) -> &EnableBits {
        &self.enable_bits[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x14 - Interrupt source enable bits"]
    #[inline(always)]
    pub fn enable_bits_iter(&self) -> impl Iterator<Item = &EnableBits> {
        self.enable_bits.iter()
    }
}
#[doc = "enable_bits (rw) register accessor: Interrupt source enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_bits::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_bits::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_bits`]
module"]
#[doc(alias = "enable_bits")]
pub type EnableBits = crate::Reg<enable_bits::EnableBitsSpec>;
#[doc = "Interrupt source enable bits"]
pub mod enable_bits;
