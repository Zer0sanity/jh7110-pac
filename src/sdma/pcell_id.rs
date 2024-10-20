#[repr(C)]
#[doc = "DMAC PrimeCell ID registers - You can treat the registers conceptually as a 32-bit register. The register is a standard cross-peripheral identification system. The DMACPCellID Register is set to 0xB105F00D."]
#[doc(alias = "pcell_id")]
pub struct PcellId {
    pcell_id0: PcellId0,
    pcell_id1: PcellId1,
    pcell_id2: PcellId2,
    pcell_id3: PcellId3,
}
impl PcellId {
    #[doc = "0x00 - DMA PrimeCell ID 0 register - is hard-coded and the fields in the register determine the reset value."]
    #[inline(always)]
    pub const fn pcell_id0(&self) -> &PcellId0 {
        &self.pcell_id0
    }
    #[doc = "0x04 - DMA PrimeCell ID 1 register - is hard-coded and the fields in the register determine the reset value."]
    #[inline(always)]
    pub const fn pcell_id1(&self) -> &PcellId1 {
        &self.pcell_id1
    }
    #[doc = "0x08 - DMA PrimeCell ID 2 register - is hard-coded and the fields in the register determine the reset value."]
    #[inline(always)]
    pub const fn pcell_id2(&self) -> &PcellId2 {
        &self.pcell_id2
    }
    #[doc = "0x0c - DMA PrimeCell ID 3 register - is hard-coded and the fields in the register determine the reset value."]
    #[inline(always)]
    pub const fn pcell_id3(&self) -> &PcellId3 {
        &self.pcell_id3
    }
}
#[doc = "pcell_id0 (r) register accessor: DMA PrimeCell ID 0 register - is hard-coded and the fields in the register determine the reset value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcell_id0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcell_id0`]
module"]
#[doc(alias = "pcell_id0")]
pub type PcellId0 = crate::Reg<pcell_id0::PcellId0Spec>;
#[doc = "DMA PrimeCell ID 0 register - is hard-coded and the fields in the register determine the reset value."]
pub mod pcell_id0;
#[doc = "pcell_id1 (r) register accessor: DMA PrimeCell ID 1 register - is hard-coded and the fields in the register determine the reset value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcell_id1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcell_id1`]
module"]
#[doc(alias = "pcell_id1")]
pub type PcellId1 = crate::Reg<pcell_id1::PcellId1Spec>;
#[doc = "DMA PrimeCell ID 1 register - is hard-coded and the fields in the register determine the reset value."]
pub mod pcell_id1;
#[doc = "pcell_id2 (r) register accessor: DMA PrimeCell ID 2 register - is hard-coded and the fields in the register determine the reset value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcell_id2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcell_id2`]
module"]
#[doc(alias = "pcell_id2")]
pub type PcellId2 = crate::Reg<pcell_id2::PcellId2Spec>;
#[doc = "DMA PrimeCell ID 2 register - is hard-coded and the fields in the register determine the reset value."]
pub mod pcell_id2;
#[doc = "pcell_id3 (r) register accessor: DMA PrimeCell ID 3 register - is hard-coded and the fields in the register determine the reset value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcell_id3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcell_id3`]
module"]
#[doc(alias = "pcell_id3")]
pub type PcellId3 = crate::Reg<pcell_id3::PcellId3Spec>;
#[doc = "DMA PrimeCell ID 3 register - is hard-coded and the fields in the register determine the reset value."]
pub mod pcell_id3;
