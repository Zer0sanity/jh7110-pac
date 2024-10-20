#[doc = "Register `stg_syscfg_163` reader"]
pub type R = crate::R<StgSyscfg163Spec>;
#[doc = "Register `stg_syscfg_163` writer"]
pub type W = crate::W<StgSyscfg163Spec>;
#[doc = "Field `u1_pcie_k_phyparam_191_160` reader - PCIE PHY Parameter (little-endian)"]
pub type U1PcieKPhyparam191_160R = crate::FieldReader<u32>;
#[doc = "Field `u1_pcie_k_phyparam_191_160` writer - PCIE PHY Parameter (little-endian)"]
pub type U1PcieKPhyparam191_160W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE PHY Parameter (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_k_phyparam_191_160(&self) -> U1PcieKPhyparam191_160R {
        U1PcieKPhyparam191_160R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PCIE PHY Parameter (little-endian)"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_k_phyparam_191_160(&mut self) -> U1PcieKPhyparam191_160W<StgSyscfg163Spec> {
        U1PcieKPhyparam191_160W::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 652\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_163::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_163::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg163Spec;
impl crate::RegisterSpec for StgSyscfg163Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_163::R`](R) reader structure"]
impl crate::Readable for StgSyscfg163Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_163::W`](W) writer structure"]
impl crate::Writable for StgSyscfg163Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_163 to value 0"]
impl crate::Resettable for StgSyscfg163Spec {
    const RESET_VALUE: u32 = 0;
}
