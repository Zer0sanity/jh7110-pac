#[doc = "Register `stg_syscfg_110` reader"]
pub type R = crate::R<StgSyscfg110Spec>;
#[doc = "Field `u0_pcie_test_out_127_96` reader - PCIE Test OUT (little-endian)"]
pub type U0PcieTestOut127_96R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_127_96(&self) -> U0PcieTestOut127_96R {
        U0PcieTestOut127_96R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 440\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_110::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg110Spec;
impl crate::RegisterSpec for StgSyscfg110Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_110::R`](R) reader structure"]
impl crate::Readable for StgSyscfg110Spec {}
#[doc = "`reset()` method sets stg_syscfg_110 to value 0"]
impl crate::Resettable for StgSyscfg110Spec {
    const RESET_VALUE: u32 = 0;
}
