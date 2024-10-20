#[doc = "Register `stg_syscfg_221` reader"]
pub type R = crate::R<StgSyscfg221Spec>;
#[doc = "Field `u1_pcie_test_out_223_192` reader - PCIE Test OUT (little-endian)"]
pub type U1PcieTestOut223_192R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_223_192(&self) -> U1PcieTestOut223_192R {
        U1PcieTestOut223_192R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 884\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_221::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg221Spec;
impl crate::RegisterSpec for StgSyscfg221Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_221::R`](R) reader structure"]
impl crate::Readable for StgSyscfg221Spec {}
#[doc = "`reset()` method sets stg_syscfg_221 to value 0"]
impl crate::Resettable for StgSyscfg221Spec {
    const RESET_VALUE: u32 = 0;
}
