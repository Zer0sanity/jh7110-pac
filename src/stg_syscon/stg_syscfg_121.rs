#[doc = "Register `stg_syscfg_121` reader"]
pub type R = crate::R<StgSyscfg121Spec>;
#[doc = "Field `u0_pcie_test_out_479_448` reader - PCIE Test OUT (little-endian)"]
pub type U0PcieTestOut479_448R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_479_448(&self) -> U0PcieTestOut479_448R {
        U0PcieTestOut479_448R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 484\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_121::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg121Spec;
impl crate::RegisterSpec for StgSyscfg121Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_121::R`](R) reader structure"]
impl crate::Readable for StgSyscfg121Spec {}
#[doc = "`reset()` method sets stg_syscfg_121 to value 0"]
impl crate::Resettable for StgSyscfg121Spec {
    const RESET_VALUE: u32 = 0;
}
