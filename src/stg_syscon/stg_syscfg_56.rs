#[doc = "Register `stg_syscfg_56` reader"]
pub type R = crate::R<StgSyscfg56Spec>;
#[doc = "Register `stg_syscfg_56` writer"]
pub type W = crate::W<StgSyscfg56Spec>;
#[doc = "Field `u0_pcie_k_phyparam_223_192` reader - PCIE PHY Parameter (little-endian)"]
pub type U0PcieKPhyparam223_192R = crate::FieldReader<u32>;
#[doc = "Field `u0_pcie_k_phyparam_223_192` writer - PCIE PHY Parameter (little-endian)"]
pub type U0PcieKPhyparam223_192W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE PHY Parameter (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_k_phyparam_223_192(&self) -> U0PcieKPhyparam223_192R {
        U0PcieKPhyparam223_192R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PCIE PHY Parameter (little-endian)"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_k_phyparam_223_192(&mut self) -> U0PcieKPhyparam223_192W<StgSyscfg56Spec> {
        U0PcieKPhyparam223_192W::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 224\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_56::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_56::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg56Spec;
impl crate::RegisterSpec for StgSyscfg56Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_56::R`](R) reader structure"]
impl crate::Readable for StgSyscfg56Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_56::W`](W) writer structure"]
impl crate::Writable for StgSyscfg56Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_56 to value 0"]
impl crate::Resettable for StgSyscfg56Spec {
    const RESET_VALUE: u32 = 0;
}