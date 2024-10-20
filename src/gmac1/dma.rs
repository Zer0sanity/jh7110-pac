#[repr(C)]
#[doc = "DMA registers"]
#[doc(alias = "dma")]
pub struct Dma {
    safety_int_status: SafetyIntStatus,
    ecc_int: EccInt,
}
impl Dma {
    #[doc = "0x00 - DMA Safety Interrupt Status"]
    #[inline(always)]
    pub const fn safety_int_status(&self) -> &SafetyIntStatus {
        &self.safety_int_status
    }
    #[doc = "0x04..0x0c - DMA ECC Interrupt registers"]
    #[inline(always)]
    pub const fn ecc_int(&self) -> &EccInt {
        &self.ecc_int
    }
}
#[doc = "safety_int_status (rw) register accessor: DMA Safety Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`safety_int_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`safety_int_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@safety_int_status`]
module"]
#[doc(alias = "safety_int_status")]
pub type SafetyIntStatus = crate::Reg<safety_int_status::SafetyIntStatusSpec>;
#[doc = "DMA Safety Interrupt Status"]
pub mod safety_int_status;
#[doc = "DMA ECC Interrupt registers"]
pub use self::ecc_int::EccInt;
#[doc = r"Cluster"]
#[doc = "DMA ECC Interrupt registers"]
pub mod ecc_int;
