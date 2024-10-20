#[repr(C)]
#[doc = "DMAC Peripheral ID registers - You can treat the registers conceptually as a 32-bit register. These read-only registers provide the following peripheral options :: PartNumber\\[11:0\\]
This identifies the peripheral. The three digit product code 0x080 is used. :: Designer ID\\[19:12\\]
This is the identification of the designer. ARM Limited is 0x41, (ASCII A). :: Revision\\[23:20\\]
This is the revision number of the peripheral. The revision number starts from 0. :: Configuration\\[31:24\\]
This is the configuration option of the peripheral."]
#[doc(alias = "periph_id")]
pub struct PeriphId {
    periph_id0: PeriphId0,
    periph_id1: PeriphId1,
    periph_id2: PeriphId2,
    periph_id3: PeriphId3,
}
impl PeriphId {
    #[doc = "0x00 - DMA Peripheral ID 0 register - is hard-coded and the fields in the register determine the reset value."]
    #[inline(always)]
    pub const fn periph_id0(&self) -> &PeriphId0 {
        &self.periph_id0
    }
    #[doc = "0x04 - DMA Peripheral ID 1 register - is hard-coded and the fields in the register determine the reset value."]
    #[inline(always)]
    pub const fn periph_id1(&self) -> &PeriphId1 {
        &self.periph_id1
    }
    #[doc = "0x08 - DMA Peripheral ID 2 register - is hard-coded and the fields in the register determine the reset value."]
    #[inline(always)]
    pub const fn periph_id2(&self) -> &PeriphId2 {
        &self.periph_id2
    }
    #[doc = "0x0c - DMA Peripheral ID 3 register - is hard-coded and the fields in the register determine the reset value."]
    #[inline(always)]
    pub const fn periph_id3(&self) -> &PeriphId3 {
        &self.periph_id3
    }
}
#[doc = "periph_id0 (r) register accessor: DMA Peripheral ID 0 register - is hard-coded and the fields in the register determine the reset value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periph_id0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id0`]
module"]
#[doc(alias = "periph_id0")]
pub type PeriphId0 = crate::Reg<periph_id0::PeriphId0Spec>;
#[doc = "DMA Peripheral ID 0 register - is hard-coded and the fields in the register determine the reset value."]
pub mod periph_id0;
#[doc = "periph_id1 (r) register accessor: DMA Peripheral ID 1 register - is hard-coded and the fields in the register determine the reset value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periph_id1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id1`]
module"]
#[doc(alias = "periph_id1")]
pub type PeriphId1 = crate::Reg<periph_id1::PeriphId1Spec>;
#[doc = "DMA Peripheral ID 1 register - is hard-coded and the fields in the register determine the reset value."]
pub mod periph_id1;
#[doc = "periph_id2 (r) register accessor: DMA Peripheral ID 2 register - is hard-coded and the fields in the register determine the reset value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periph_id2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id2`]
module"]
#[doc(alias = "periph_id2")]
pub type PeriphId2 = crate::Reg<periph_id2::PeriphId2Spec>;
#[doc = "DMA Peripheral ID 2 register - is hard-coded and the fields in the register determine the reset value."]
pub mod periph_id2;
#[doc = "periph_id3 (r) register accessor: DMA Peripheral ID 3 register - is hard-coded and the fields in the register determine the reset value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periph_id3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id3`]
module"]
#[doc(alias = "periph_id3")]
pub type PeriphId3 = crate::Reg<periph_id3::PeriphId3Spec>;
#[doc = "DMA Peripheral ID 3 register - is hard-coded and the fields in the register determine the reset value."]
pub mod periph_id3;
