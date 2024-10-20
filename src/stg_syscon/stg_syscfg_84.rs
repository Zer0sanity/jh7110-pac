#[doc = "Register `stg_syscfg_84` reader"]
pub type R = crate::R<StgSyscfg84Spec>;
#[doc = "Register `stg_syscfg_84` writer"]
pub type W = crate::W<StgSyscfg84Spec>;
#[doc = "Field `u0_pcie_pl_sideband_in_31_0` reader - PCIE PL Sideband IN (little-endian)"]
pub type U0PciePlSidebandIn31_0R = crate::FieldReader<u32>;
#[doc = "Field `u0_pcie_pl_sideband_in_31_0` writer - PCIE PL Sideband IN (little-endian)"]
pub type U0PciePlSidebandIn31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE PL Sideband IN (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_pl_sideband_in_31_0(&self) -> U0PciePlSidebandIn31_0R {
        U0PciePlSidebandIn31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PCIE PL Sideband IN (little-endian)"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_pl_sideband_in_31_0(&mut self) -> U0PciePlSidebandIn31_0W<StgSyscfg84Spec> {
        U0PciePlSidebandIn31_0W::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 336\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_84::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_84::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg84Spec;
impl crate::RegisterSpec for StgSyscfg84Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_84::R`](R) reader structure"]
impl crate::Readable for StgSyscfg84Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_84::W`](W) writer structure"]
impl crate::Writable for StgSyscfg84Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_84 to value 0"]
impl crate::Resettable for StgSyscfg84Spec {
    const RESET_VALUE: u32 = 0;
}
