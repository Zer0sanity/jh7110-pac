#[doc = "Register `stg_syscfg_226` reader"]
pub type R = crate::R<StgSyscfg226Spec>;
#[doc = "Field `u1_pcie_test_out_383_352` reader - PCIE Test OUT (little-endian)"]
pub type U1PcieTestOut383_352R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_383_352(&self) -> U1PcieTestOut383_352R {
        U1PcieTestOut383_352R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 904\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_226::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg226Spec;
impl crate::RegisterSpec for StgSyscfg226Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_226::R`](R) reader structure"]
impl crate::Readable for StgSyscfg226Spec {}
#[doc = "`reset()` method sets stg_syscfg_226 to value 0"]
impl crate::Resettable for StgSyscfg226Spec {
    const RESET_VALUE: u32 = 0;
}
