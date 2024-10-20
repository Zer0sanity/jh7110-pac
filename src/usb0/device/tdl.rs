#[repr(C)]
#[doc = "USB3 Device TDL registers."]
#[doc(alias = "tdl")]
pub struct Tdl {
    cfg_from_trb: CfgFromTrb,
    beh: Beh,
    ep: Ep,
    beh2: Beh2,
    dma_adv: DmaAdv,
}
impl Tdl {
    #[doc = "0x00 - TDL configuration source."]
    #[inline(always)]
    pub const fn cfg_from_trb(&self) -> &CfgFromTrb {
        &self.cfg_from_trb
    }
    #[doc = "0x04 - TDL behavior configuration."]
    #[inline(always)]
    pub const fn beh(&self) -> &Beh {
        &self.beh
    }
    #[doc = "0x08 - TDL endpoint configuration."]
    #[inline(always)]
    pub const fn ep(&self) -> &Ep {
        &self.ep
    }
    #[doc = "0x0c - TDL behavior 2 configuration."]
    #[inline(always)]
    pub const fn beh2(&self) -> &Beh2 {
        &self.beh2
    }
    #[doc = "0x10 - TDL DMA Advance configuration."]
    #[inline(always)]
    pub const fn dma_adv(&self) -> &DmaAdv {
        &self.dma_adv
    }
}
#[doc = "cfg_from_trb (rw) register accessor: TDL configuration source.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_from_trb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_from_trb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_from_trb`]
module"]
#[doc(alias = "cfg_from_trb")]
pub type CfgFromTrb = crate::Reg<cfg_from_trb::CfgFromTrbSpec>;
#[doc = "TDL configuration source."]
pub mod cfg_from_trb;
#[doc = "beh (rw) register accessor: TDL behavior configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`beh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`beh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@beh`]
module"]
#[doc(alias = "beh")]
pub type Beh = crate::Reg<beh::BehSpec>;
#[doc = "TDL behavior configuration."]
pub mod beh;
#[doc = "ep (rw) register accessor: TDL endpoint configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep`]
module"]
#[doc(alias = "ep")]
pub type Ep = crate::Reg<ep::EpSpec>;
#[doc = "TDL endpoint configuration."]
pub mod ep;
#[doc = "beh2 (rw) register accessor: TDL behavior 2 configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`beh2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`beh2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@beh2`]
module"]
#[doc(alias = "beh2")]
pub type Beh2 = crate::Reg<beh2::Beh2Spec>;
#[doc = "TDL behavior 2 configuration."]
pub mod beh2;
#[doc = "dma_adv (rw) register accessor: TDL DMA Advance configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_adv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_adv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_adv`]
module"]
#[doc(alias = "dma_adv")]
pub type DmaAdv = crate::Reg<dma_adv::DmaAdvSpec>;
#[doc = "TDL DMA Advance configuration."]
pub mod dma_adv;
