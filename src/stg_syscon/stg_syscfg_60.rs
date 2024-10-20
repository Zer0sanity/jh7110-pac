#[doc = "Register `stg_syscfg_60` reader"]
pub type R = crate::R<StgSyscfg60Spec>;
#[doc = "Register `stg_syscfg_60` writer"]
pub type W = crate::W<StgSyscfg60Spec>;
#[doc = "Field `u0_pcie_k_phyparam_351_320` reader - PCIE PHY Parameter (little-endian)"]
pub type U0PcieKPhyparam351_320R = crate::FieldReader<u32>;
#[doc = "Field `u0_pcie_k_phyparam_351_320` writer - PCIE PHY Parameter (little-endian)"]
pub type U0PcieKPhyparam351_320W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE PHY Parameter (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_k_phyparam_351_320(&self) -> U0PcieKPhyparam351_320R {
        U0PcieKPhyparam351_320R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PCIE PHY Parameter (little-endian)"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_k_phyparam_351_320(&mut self) -> U0PcieKPhyparam351_320W<StgSyscfg60Spec> {
        U0PcieKPhyparam351_320W::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 240\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_60::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_60::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg60Spec;
impl crate::RegisterSpec for StgSyscfg60Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_60::R`](R) reader structure"]
impl crate::Readable for StgSyscfg60Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_60::W`](W) writer structure"]
impl crate::Writable for StgSyscfg60Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_60 to value 0"]
impl crate::Resettable for StgSyscfg60Spec {
    const RESET_VALUE: u32 = 0;
}
