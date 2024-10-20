#[doc = "Register `stg_syscfg_114` reader"]
pub type R = crate::R<StgSyscfg114Spec>;
#[doc = "Field `u0_pcie_test_out_255_224` reader - PCIE Test OUT (little-endian)"]
pub type U0PcieTestOut255_224R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_255_224(&self) -> U0PcieTestOut255_224R {
        U0PcieTestOut255_224R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 456\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_114::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg114Spec;
impl crate::RegisterSpec for StgSyscfg114Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_114::R`](R) reader structure"]
impl crate::Readable for StgSyscfg114Spec {}
#[doc = "`reset()` method sets stg_syscfg_114 to value 0"]
impl crate::Resettable for StgSyscfg114Spec {
    const RESET_VALUE: u32 = 0;
}
