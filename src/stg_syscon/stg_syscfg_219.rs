#[doc = "Register `stg_syscfg_219` reader"]
pub type R = crate::R<StgSyscfg219Spec>;
#[doc = "Field `u1_pcie_test_out_159_128` reader - PCIE Test OUT (little-endian)"]
pub type U1PcieTestOut159_128R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_159_128(&self) -> U1PcieTestOut159_128R {
        U1PcieTestOut159_128R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 876\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_219::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg219Spec;
impl crate::RegisterSpec for StgSyscfg219Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_219::R`](R) reader structure"]
impl crate::Readable for StgSyscfg219Spec {}
#[doc = "`reset()` method sets stg_syscfg_219 to value 0"]
impl crate::Resettable for StgSyscfg219Spec {
    const RESET_VALUE: u32 = 0;
}
