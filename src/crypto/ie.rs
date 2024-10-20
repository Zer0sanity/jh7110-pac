#[repr(C)]
#[doc = "JH7110 Crypto Interrupt Enable registers"]
#[doc(alias = "ie")]
pub struct Ie {
    mask: Mask,
    flag: Flag,
}
impl Ie {
    #[doc = "0x00 - JH7110 Crypto Interrupt Enable Mask"]
    #[inline(always)]
    pub const fn mask(&self) -> &Mask {
        &self.mask
    }
    #[doc = "0x04 - JH7110 Crypto Interrupt Enable Flag"]
    #[inline(always)]
    pub const fn flag(&self) -> &Flag {
        &self.flag
    }
}
#[doc = "mask (rw) register accessor: JH7110 Crypto Interrupt Enable Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask`]
module"]
#[doc(alias = "mask")]
pub type Mask = crate::Reg<mask::MaskSpec>;
#[doc = "JH7110 Crypto Interrupt Enable Mask"]
pub mod mask;
#[doc = "flag (rw) register accessor: JH7110 Crypto Interrupt Enable Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flag`]
module"]
#[doc(alias = "flag")]
pub type Flag = crate::Reg<flag::FlagSpec>;
#[doc = "JH7110 Crypto Interrupt Enable Flag"]
pub mod flag;
