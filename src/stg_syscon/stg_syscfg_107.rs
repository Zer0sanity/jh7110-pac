#[doc = "Register `stg_syscfg_107` reader"]
pub type R = crate::R<StgSyscfg107Spec>;
#[doc = "Field `u0_pcie_test_out_31_0` reader - PCIE Test OUT (little-endian)"]
pub type U0PcieTestOut31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_31_0(&self) -> U0PcieTestOut31_0R {
        U0PcieTestOut31_0R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 428\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_107::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg107Spec;
impl crate::RegisterSpec for StgSyscfg107Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_107::R`](R) reader structure"]
impl crate::Readable for StgSyscfg107Spec {}
#[doc = "`reset()` method sets stg_syscfg_107 to value 0"]
impl crate::Resettable for StgSyscfg107Spec {
    const RESET_VALUE: u32 = 0;
}
