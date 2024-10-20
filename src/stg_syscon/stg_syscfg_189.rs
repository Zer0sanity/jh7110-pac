#[doc = "Register `stg_syscfg_189` reader"]
pub type R = crate::R<StgSyscfg189Spec>;
#[doc = "Register `stg_syscfg_189` writer"]
pub type W = crate::W<StgSyscfg189Spec>;
#[doc = "Field `u1_pcie_pf2_offset` reader - PCIE PF Offset"]
pub type U1PciePf2OffsetR = crate::FieldReader<u32>;
#[doc = "Field `u1_pcie_pf2_offset` writer - PCIE PF Offset"]
pub type U1PciePf2OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - PCIE PF Offset"]
    #[inline(always)]
    pub fn u1_pcie_pf2_offset(&self) -> U1PciePf2OffsetR {
        U1PciePf2OffsetR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - PCIE PF Offset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_pf2_offset(&mut self) -> U1PciePf2OffsetW<StgSyscfg189Spec> {
        U1PciePf2OffsetW::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 756\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_189::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_189::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg189Spec;
impl crate::RegisterSpec for StgSyscfg189Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_189::R`](R) reader structure"]
impl crate::Readable for StgSyscfg189Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_189::W`](W) writer structure"]
impl crate::Writable for StgSyscfg189Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_189 to value 0"]
impl crate::Resettable for StgSyscfg189Spec {
    const RESET_VALUE: u32 = 0;
}
