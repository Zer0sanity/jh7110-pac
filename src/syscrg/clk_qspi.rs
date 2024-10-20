#[repr(C)]
#[doc = "Clock QSPI registers"]
#[doc(alias = "clk_qspi")]
pub struct ClkQspi {
    ahb: Ahb,
    apb: Apb,
    clk_ref_src: ClkRefSrc,
    clk_ref: ClkRef,
}
impl ClkQspi {
    #[doc = "0x00 - Clock QSPI AHB"]
    #[inline(always)]
    pub const fn ahb(&self) -> &Ahb {
        &self.ahb
    }
    #[doc = "0x04 - Clock QSPI APB"]
    #[inline(always)]
    pub const fn apb(&self) -> &Apb {
        &self.apb
    }
    #[doc = "0x08 - Clock QSPI Reference Source"]
    #[inline(always)]
    pub const fn clk_ref_src(&self) -> &ClkRefSrc {
        &self.clk_ref_src
    }
    #[doc = "0x0c - Clock QSPI Reference"]
    #[inline(always)]
    pub const fn clk_ref(&self) -> &ClkRef {
        &self.clk_ref
    }
}
#[doc = "ahb (rw) register accessor: Clock QSPI AHB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb`]
module"]
#[doc(alias = "ahb")]
pub type Ahb = crate::Reg<ahb::AhbSpec>;
#[doc = "Clock QSPI AHB"]
pub mod ahb;
#[doc = "apb (rw) register accessor: Clock QSPI APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb`]
module"]
#[doc(alias = "apb")]
pub type Apb = crate::Reg<apb::ApbSpec>;
#[doc = "Clock QSPI APB"]
pub mod apb;
#[doc = "clk_ref_src (rw) register accessor: Clock QSPI Reference Source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ref_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ref_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ref_src`]
module"]
#[doc(alias = "clk_ref_src")]
pub type ClkRefSrc = crate::Reg<clk_ref_src::ClkRefSrcSpec>;
#[doc = "Clock QSPI Reference Source"]
pub mod clk_ref_src;
#[doc = "clk_ref (rw) register accessor: Clock QSPI Reference\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ref::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ref::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ref`]
module"]
#[doc(alias = "clk_ref")]
pub type ClkRef = crate::Reg<clk_ref::ClkRefSpec>;
#[doc = "Clock QSPI Reference"]
pub mod clk_ref;
