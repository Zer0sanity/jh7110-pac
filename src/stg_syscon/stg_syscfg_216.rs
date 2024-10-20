#[doc = "Register `stg_syscfg_216` reader"]
pub type R = crate::R<StgSyscfg216Spec>;
#[doc = "Field `u1_pcie_test_out_63_32` reader - PCIE Test OUT (little-endian)"]
pub type U1PcieTestOut63_32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_63_32(&self) -> U1PcieTestOut63_32R {
        U1PcieTestOut63_32R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 864\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_216::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg216Spec;
impl crate::RegisterSpec for StgSyscfg216Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_216::R`](R) reader structure"]
impl crate::Readable for StgSyscfg216Spec {}
#[doc = "`reset()` method sets stg_syscfg_216 to value 0"]
impl crate::Resettable for StgSyscfg216Spec {
    const RESET_VALUE: u32 = 0;
}
