#[doc = "Register `stg_syscfg_230` reader"]
pub type R = crate::R<StgSyscfg230Spec>;
#[doc = "Field `u1_pcie_test_out_511_480` reader - PCIE Test OUT (little-endian)"]
pub type U1PcieTestOut511_480R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_511_480(&self) -> U1PcieTestOut511_480R {
        U1PcieTestOut511_480R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 920\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_230::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg230Spec;
impl crate::RegisterSpec for StgSyscfg230Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_230::R`](R) reader structure"]
impl crate::Readable for StgSyscfg230Spec {}
#[doc = "`reset()` method sets stg_syscfg_230 to value 0"]
impl crate::Resettable for StgSyscfg230Spec {
    const RESET_VALUE: u32 = 0;
}
