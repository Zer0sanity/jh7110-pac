#[doc = "Register `stg_syscfg_83` reader"]
pub type R = crate::R<StgSyscfg83Spec>;
#[doc = "Field `u0_pcie_pl_pclk_rate` reader - PCIE PL PCLK Rate"]
pub type U0PciePlPclkRateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - PCIE PL PCLK Rate"]
    #[inline(always)]
    pub fn u0_pcie_pl_pclk_rate(&self) -> U0PciePlPclkRateR {
        U0PciePlPclkRateR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 332\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_83::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg83Spec;
impl crate::RegisterSpec for StgSyscfg83Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_83::R`](R) reader structure"]
impl crate::Readable for StgSyscfg83Spec {}
#[doc = "`reset()` method sets stg_syscfg_83 to value 0"]
impl crate::Resettable for StgSyscfg83Spec {
    const RESET_VALUE: u32 = 0;
}
