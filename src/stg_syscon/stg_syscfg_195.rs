#[doc = "Register `stg_syscfg_195` reader"]
pub type R = crate::R<StgSyscfg195Spec>;
#[doc = "Field `u1_pcie_pl_sideband_out_63_32` reader - PCIE PL Sideband OUT (little-endian)"]
pub type U1PciePlSidebandOut63_32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE PL Sideband OUT (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_pl_sideband_out_63_32(&self) -> U1PciePlSidebandOut63_32R {
        U1PciePlSidebandOut63_32R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 780\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_195::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg195Spec;
impl crate::RegisterSpec for StgSyscfg195Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_195::R`](R) reader structure"]
impl crate::Readable for StgSyscfg195Spec {}
#[doc = "`reset()` method sets stg_syscfg_195 to value 0"]
impl crate::Resettable for StgSyscfg195Spec {
    const RESET_VALUE: u32 = 0;
}
