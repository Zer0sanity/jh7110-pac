#[doc = "Register `stg_syscfg_224` reader"]
pub type R = crate::R<StgSyscfg224Spec>;
#[doc = "Field `u1_pcie_test_out_319_288` reader - PCIE Test OUT (little-endian)"]
pub type U1PcieTestOut319_288R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_319_288(&self) -> U1PcieTestOut319_288R {
        U1PcieTestOut319_288R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 896\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_224::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg224Spec;
impl crate::RegisterSpec for StgSyscfg224Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_224::R`](R) reader structure"]
impl crate::Readable for StgSyscfg224Spec {}
#[doc = "`reset()` method sets stg_syscfg_224 to value 0"]
impl crate::Resettable for StgSyscfg224Spec {
    const RESET_VALUE: u32 = 0;
}
