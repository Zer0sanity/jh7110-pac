#[doc = "Register `stg_syscfg_81` reader"]
pub type R = crate::R<StgSyscfg81Spec>;
#[doc = "Register `stg_syscfg_81` writer"]
pub type W = crate::W<StgSyscfg81Spec>;
#[doc = "Field `u0_pcie_pf2_offset` reader - PCIE PF Offset"]
pub type U0PciePf2OffsetR = crate::FieldReader<u32>;
#[doc = "Field `u0_pcie_pf2_offset` writer - PCIE PF Offset"]
pub type U0PciePf2OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - PCIE PF Offset"]
    #[inline(always)]
    pub fn u0_pcie_pf2_offset(&self) -> U0PciePf2OffsetR {
        U0PciePf2OffsetR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - PCIE PF Offset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_pf2_offset(&mut self) -> U0PciePf2OffsetW<StgSyscfg81Spec> {
        U0PciePf2OffsetW::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 324\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_81::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_81::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg81Spec;
impl crate::RegisterSpec for StgSyscfg81Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_81::R`](R) reader structure"]
impl crate::Readable for StgSyscfg81Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_81::W`](W) writer structure"]
impl crate::Writable for StgSyscfg81Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_81 to value 0"]
impl crate::Resettable for StgSyscfg81Spec {
    const RESET_VALUE: u32 = 0;
}
