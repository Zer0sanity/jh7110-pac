#[doc = "Register `stg_syscfg_112` reader"]
pub type R = crate::R<StgSyscfg112Spec>;
#[doc = "Field `u0_pcie_test_out_191_160` reader - PCIE Test OUT (little-endian)"]
pub type U0PcieTestOut191_160R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_191_160(&self) -> U0PcieTestOut191_160R {
        U0PcieTestOut191_160R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 448\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_112::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg112Spec;
impl crate::RegisterSpec for StgSyscfg112Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_112::R`](R) reader structure"]
impl crate::Readable for StgSyscfg112Spec {}
#[doc = "`reset()` method sets stg_syscfg_112 to value 0"]
impl crate::Resettable for StgSyscfg112Spec {
    const RESET_VALUE: u32 = 0;
}
