#[doc = "Register `stg_syscfg_169` reader"]
pub type R = crate::R<StgSyscfg169Spec>;
#[doc = "Register `stg_syscfg_169` writer"]
pub type W = crate::W<StgSyscfg169Spec>;
#[doc = "Field `u1_pcie_k_phyparam_383_352` reader - PCIE PHY Parameter (little-endian)"]
pub type U1PcieKPhyparam383_352R = crate::FieldReader<u32>;
#[doc = "Field `u1_pcie_k_phyparam_383_352` writer - PCIE PHY Parameter (little-endian)"]
pub type U1PcieKPhyparam383_352W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE PHY Parameter (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_k_phyparam_383_352(&self) -> U1PcieKPhyparam383_352R {
        U1PcieKPhyparam383_352R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PCIE PHY Parameter (little-endian)"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_k_phyparam_383_352(&mut self) -> U1PcieKPhyparam383_352W<StgSyscfg169Spec> {
        U1PcieKPhyparam383_352W::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 676\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_169::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_169::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg169Spec;
impl crate::RegisterSpec for StgSyscfg169Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_169::R`](R) reader structure"]
impl crate::Readable for StgSyscfg169Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_169::W`](W) writer structure"]
impl crate::Writable for StgSyscfg169Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_169 to value 0"]
impl crate::Resettable for StgSyscfg169Spec {
    const RESET_VALUE: u32 = 0;
}
