#[doc = "Register `stg_syscfg_108` reader"]
pub type R = crate::R<StgSyscfg108Spec>;
#[doc = "Field `u0_pcie_test_out_63_32` reader - PCIE Test OUT (little-endian)"]
pub type U0PcieTestOut63_32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_63_32(&self) -> U0PcieTestOut63_32R {
        U0PcieTestOut63_32R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 432\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_108::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg108Spec;
impl crate::RegisterSpec for StgSyscfg108Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_108::R`](R) reader structure"]
impl crate::Readable for StgSyscfg108Spec {}
#[doc = "`reset()` method sets stg_syscfg_108 to value 0"]
impl crate::Resettable for StgSyscfg108Spec {
    const RESET_VALUE: u32 = 0;
}
