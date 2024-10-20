#[doc = "Register `stg_syscfg_115` reader"]
pub type R = crate::R<StgSyscfg115Spec>;
#[doc = "Field `u0_pcie_test_out_287_256` reader - PCIE Test OUT (little-endian)"]
pub type U0PcieTestOut287_256R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_287_256(&self) -> U0PcieTestOut287_256R {
        U0PcieTestOut287_256R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 460\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_115::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg115Spec;
impl crate::RegisterSpec for StgSyscfg115Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_115::R`](R) reader structure"]
impl crate::Readable for StgSyscfg115Spec {}
#[doc = "`reset()` method sets stg_syscfg_115 to value 0"]
impl crate::Resettable for StgSyscfg115Spec {
    const RESET_VALUE: u32 = 0;
}
