#[doc = "Register `stg_syscfg_222` reader"]
pub type R = crate::R<StgSyscfg222Spec>;
#[doc = "Field `u1_pcie_test_out_255_224` reader - PCIE Test OUT (little-endian)"]
pub type U1PcieTestOut255_224R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_255_224(&self) -> U1PcieTestOut255_224R {
        U1PcieTestOut255_224R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 888\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_222::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg222Spec;
impl crate::RegisterSpec for StgSyscfg222Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_222::R`](R) reader structure"]
impl crate::Readable for StgSyscfg222Spec {}
#[doc = "`reset()` method sets stg_syscfg_222 to value 0"]
impl crate::Resettable for StgSyscfg222Spec {
    const RESET_VALUE: u32 = 0;
}
