#[repr(C)]
#[doc = "DMAC Test registers - controls and reads registers used in peripheral integration tests."]
#[doc(alias = "test")]
pub struct Test {
    itcr: Itcr,
    itop1: Itop1,
    itop2: Itop2,
    itop3: Itop3,
}
impl Test {
    #[doc = "0x00 - DMA Test Control register - enables you to test the DMAC using TIC block-level tests and Built-In Self-Test (BIST) integration and system level tests."]
    #[inline(always)]
    pub const fn itcr(&self) -> &Itcr {
        &self.itcr
    }
    #[doc = "0x04 - DMA Integration Test Output 1 register - controls and reads the DMACCLR\\[15:0\\]
output lines in test mode."]
    #[inline(always)]
    pub const fn itop1(&self) -> &Itop1 {
        &self.itop1
    }
    #[doc = "0x08 - DMA Integration Test Output 2 register - controls and reads the DMACTC\\[15:0\\]
output lines in test mode."]
    #[inline(always)]
    pub const fn itop2(&self) -> &Itop2 {
        &self.itop2
    }
    #[doc = "0x0c - DMA Integration Test Output 3 register - controls and reads the interrupt request output lines in test mode. The DMACINTR interrupt request signal combines both interrupt requests, DMACINTTC and DMACINTERR, into one interrupt request signal. Therefore, if you set either the TC or E bits, then DMACINTR is active."]
    #[inline(always)]
    pub const fn itop3(&self) -> &Itop3 {
        &self.itop3
    }
}
#[doc = "itcr (rw) register accessor: DMA Test Control register - enables you to test the DMAC using TIC block-level tests and Built-In Self-Test (BIST) integration and system level tests.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itcr`]
module"]
#[doc(alias = "itcr")]
pub type Itcr = crate::Reg<itcr::ItcrSpec>;
#[doc = "DMA Test Control register - enables you to test the DMAC using TIC block-level tests and Built-In Self-Test (BIST) integration and system level tests."]
pub mod itcr;
#[doc = "itop1 (rw) register accessor: DMA Integration Test Output 1 register - controls and reads the DMACCLR\\[15:0\\]
output lines in test mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itop1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itop1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itop1`]
module"]
#[doc(alias = "itop1")]
pub type Itop1 = crate::Reg<itop1::Itop1Spec>;
#[doc = "DMA Integration Test Output 1 register - controls and reads the DMACCLR\\[15:0\\]
output lines in test mode."]
pub mod itop1;
#[doc = "itop2 (rw) register accessor: DMA Integration Test Output 2 register - controls and reads the DMACTC\\[15:0\\]
output lines in test mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itop2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itop2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itop2`]
module"]
#[doc(alias = "itop2")]
pub type Itop2 = crate::Reg<itop2::Itop2Spec>;
#[doc = "DMA Integration Test Output 2 register - controls and reads the DMACTC\\[15:0\\]
output lines in test mode."]
pub mod itop2;
#[doc = "itop3 (rw) register accessor: DMA Integration Test Output 3 register - controls and reads the interrupt request output lines in test mode. The DMACINTR interrupt request signal combines both interrupt requests, DMACINTTC and DMACINTERR, into one interrupt request signal. Therefore, if you set either the TC or E bits, then DMACINTR is active.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itop3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itop3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itop3`]
module"]
#[doc(alias = "itop3")]
pub type Itop3 = crate::Reg<itop3::Itop3Spec>;
#[doc = "DMA Integration Test Output 3 register - controls and reads the interrupt request output lines in test mode. The DMACINTR interrupt request signal combines both interrupt requests, DMACINTTC and DMACINTERR, into one interrupt request signal. Therefore, if you set either the TC or E bits, then DMACINTR is active."]
pub mod itop3;
