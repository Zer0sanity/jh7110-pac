#[repr(C)]
#[doc = "Clock PDM"]
#[doc(alias = "clk_pdm")]
pub struct ClkPdm {
    dmic: Dmic,
    apb: Apb,
}
impl ClkPdm {
    #[doc = "0x00 - Clock PDM DMIC"]
    #[inline(always)]
    pub const fn dmic(&self) -> &Dmic {
        &self.dmic
    }
    #[doc = "0x04 - Clock PDM APB"]
    #[inline(always)]
    pub const fn apb(&self) -> &Apb {
        &self.apb
    }
}
#[doc = "dmic (rw) register accessor: Clock PDM DMIC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmic::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmic::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmic`]
module"]
#[doc(alias = "dmic")]
pub type Dmic = crate::Reg<dmic::DmicSpec>;
#[doc = "Clock PDM DMIC"]
pub mod dmic;
#[doc = "apb (rw) register accessor: Clock PDM APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb`]
module"]
#[doc(alias = "apb")]
pub type Apb = crate::Reg<apb::ApbSpec>;
#[doc = "Clock PDM APB"]
pub mod apb;
