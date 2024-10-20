#[doc = "Register `stg_syscfg_109` reader"]
pub type R = crate::R<StgSyscfg109Spec>;
#[doc = "Field `u0_pcie_test_out_95_64` reader - PCIE Test OUT (little-endian)"]
pub type U0PcieTestOut95_64R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_95_64(&self) -> U0PcieTestOut95_64R {
        U0PcieTestOut95_64R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 436\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_109::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg109Spec;
impl crate::RegisterSpec for StgSyscfg109Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_109::R`](R) reader structure"]
impl crate::Readable for StgSyscfg109Spec {}
#[doc = "`reset()` method sets stg_syscfg_109 to value 0"]
impl crate::Resettable for StgSyscfg109Spec {
    const RESET_VALUE: u32 = 0;
}
