#[doc = "Register `stg_syscfg_188` reader"]
pub type R = crate::R<StgSyscfg188Spec>;
#[doc = "Register `stg_syscfg_188` writer"]
pub type W = crate::W<StgSyscfg188Spec>;
#[doc = "Field `u1_pcie_pf1_offset` reader - PCIE PF Offset"]
pub type U1PciePf1OffsetR = crate::FieldReader<u32>;
#[doc = "Field `u1_pcie_pf1_offset` writer - PCIE PF Offset"]
pub type U1PciePf1OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - PCIE PF Offset"]
    #[inline(always)]
    pub fn u1_pcie_pf1_offset(&self) -> U1PciePf1OffsetR {
        U1PciePf1OffsetR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - PCIE PF Offset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_pf1_offset(&mut self) -> U1PciePf1OffsetW<StgSyscfg188Spec> {
        U1PciePf1OffsetW::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 752\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_188::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_188::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg188Spec;
impl crate::RegisterSpec for StgSyscfg188Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_188::R`](R) reader structure"]
impl crate::Readable for StgSyscfg188Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_188::W`](W) writer structure"]
impl crate::Writable for StgSyscfg188Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_188 to value 0"]
impl crate::Resettable for StgSyscfg188Spec {
    const RESET_VALUE: u32 = 0;
}
