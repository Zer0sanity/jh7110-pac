#[doc = "Register `stg_syscfg_217` reader"]
pub type R = crate::R<StgSyscfg217Spec>;
#[doc = "Field `u1_pcie_test_out_95_64` reader - PCIE Test OUT (little-endian)"]
pub type U1PcieTestOut95_64R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_95_64(&self) -> U1PcieTestOut95_64R {
        U1PcieTestOut95_64R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 868\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_217::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg217Spec;
impl crate::RegisterSpec for StgSyscfg217Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_217::R`](R) reader structure"]
impl crate::Readable for StgSyscfg217Spec {}
#[doc = "`reset()` method sets stg_syscfg_217 to value 0"]
impl crate::Resettable for StgSyscfg217Spec {
    const RESET_VALUE: u32 = 0;
}
