#[doc = "Register `stg_syscfg_118` reader"]
pub type R = crate::R<StgSyscfg118Spec>;
#[doc = "Field `u0_pcie_test_out_383_352` reader - PCIE Test OUT (little-endian)"]
pub type U0PcieTestOut383_352R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_383_352(&self) -> U0PcieTestOut383_352R {
        U0PcieTestOut383_352R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 472\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_118::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg118Spec;
impl crate::RegisterSpec for StgSyscfg118Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_118::R`](R) reader structure"]
impl crate::Readable for StgSyscfg118Spec {}
#[doc = "`reset()` method sets stg_syscfg_118 to value 0"]
impl crate::Resettable for StgSyscfg118Spec {
    const RESET_VALUE: u32 = 0;
}
