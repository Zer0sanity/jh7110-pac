#[repr(C)]
#[doc = "Clock TDM"]
#[doc(alias = "clk_tdm")]
pub struct ClkTdm {
    ahb: Ahb,
    apb: Apb,
    internal: Internal,
    tdm: Tdm,
    tdm_neg: TdmNeg,
}
impl ClkTdm {
    #[doc = "0x00 - Clock TDM AHB"]
    #[inline(always)]
    pub const fn ahb(&self) -> &Ahb {
        &self.ahb
    }
    #[doc = "0x04 - Clock TDM APB"]
    #[inline(always)]
    pub const fn apb(&self) -> &Apb {
        &self.apb
    }
    #[doc = "0x08 - Clock TDM Internal"]
    #[inline(always)]
    pub const fn internal(&self) -> &Internal {
        &self.internal
    }
    #[doc = "0x0c - Clock TDM (clock selector)"]
    #[inline(always)]
    pub const fn tdm(&self) -> &Tdm {
        &self.tdm
    }
    #[doc = "0x10 - Clock TDM Negative"]
    #[inline(always)]
    pub const fn tdm_neg(&self) -> &TdmNeg {
        &self.tdm_neg
    }
}
#[doc = "ahb (rw) register accessor: Clock TDM AHB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb`]
module"]
#[doc(alias = "ahb")]
pub type Ahb = crate::Reg<ahb::AhbSpec>;
#[doc = "Clock TDM AHB"]
pub mod ahb;
#[doc = "apb (rw) register accessor: Clock TDM APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb`]
module"]
#[doc(alias = "apb")]
pub type Apb = crate::Reg<apb::ApbSpec>;
#[doc = "Clock TDM APB"]
pub mod apb;
#[doc = "internal (rw) register accessor: Clock TDM Internal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`internal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`internal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@internal`]
module"]
#[doc(alias = "internal")]
pub type Internal = crate::Reg<internal::InternalSpec>;
#[doc = "Clock TDM Internal"]
pub mod internal;
#[doc = "tdm (rw) register accessor: Clock TDM (clock selector)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdm`]
module"]
#[doc(alias = "tdm")]
pub type Tdm = crate::Reg<tdm::TdmSpec>;
#[doc = "Clock TDM (clock selector)"]
pub mod tdm;
#[doc = "tdm_neg (rw) register accessor: Clock TDM Negative\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdm_neg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdm_neg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdm_neg`]
module"]
#[doc(alias = "tdm_neg")]
pub type TdmNeg = crate::Reg<tdm_neg::TdmNegSpec>;
#[doc = "Clock TDM Negative"]
pub mod tdm_neg;
