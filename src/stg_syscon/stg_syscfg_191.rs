#[doc = "Register `stg_syscfg_191` reader"]
pub type R = crate::R<StgSyscfg191Spec>;
#[doc = "Field `u1_pcie_pl_pclk_rate` reader - PCIE PL PCLK Rate"]
pub type U1PciePlPclkRateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - PCIE PL PCLK Rate"]
    #[inline(always)]
    pub fn u1_pcie_pl_pclk_rate(&self) -> U1PciePlPclkRateR {
        U1PciePlPclkRateR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 764\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_191::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg191Spec;
impl crate::RegisterSpec for StgSyscfg191Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_191::R`](R) reader structure"]
impl crate::Readable for StgSyscfg191Spec {}
#[doc = "`reset()` method sets stg_syscfg_191 to value 0"]
impl crate::Resettable for StgSyscfg191Spec {
    const RESET_VALUE: u32 = 0;
}
