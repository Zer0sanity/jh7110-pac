#[doc = "Register `stg_syscfg_220` reader"]
pub type R = crate::R<StgSyscfg220Spec>;
#[doc = "Field `u1_pcie_test_out_191_160` reader - PCIE Test OUT (little-endian)"]
pub type U1PcieTestOut191_160R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_191_160(&self) -> U1PcieTestOut191_160R {
        U1PcieTestOut191_160R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 880\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_220::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg220Spec;
impl crate::RegisterSpec for StgSyscfg220Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_220::R`](R) reader structure"]
impl crate::Readable for StgSyscfg220Spec {}
#[doc = "`reset()` method sets stg_syscfg_220 to value 0"]
impl crate::Resettable for StgSyscfg220Spec {
    const RESET_VALUE: u32 = 0;
}
