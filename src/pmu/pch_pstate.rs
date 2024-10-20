#[doc = "Register `pch_pstate` reader"]
pub type R = crate::R<PchPstateSpec>;
#[doc = "Register `pch_pstate` writer"]
pub type W = crate::W<PchPstateSpec>;
#[doc = "Field `pch_pstate` reader - P-channel state set."]
pub type PchPstateR = crate::FieldReader;
#[doc = "Field `pch_pstate` writer - P-channel state set."]
pub type PchPstateW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - P-channel state set."]
    #[inline(always)]
    pub fn pch_pstate(&self) -> PchPstateR {
        PchPstateR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - P-channel state set."]
    #[inline(always)]
    #[must_use]
    pub fn pch_pstate(&mut self) -> PchPstateW<PchPstateSpec> {
        PchPstateW::new(self, 0)
    }
}
#[doc = "P-channel PSTATE register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pch_pstate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pch_pstate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PchPstateSpec;
impl crate::RegisterSpec for PchPstateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pch_pstate::R`](R) reader structure"]
impl crate::Readable for PchPstateSpec {}
#[doc = "`write(|w| ..)` method takes [`pch_pstate::W`](W) writer structure"]
impl crate::Writable for PchPstateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pch_pstate to value 0"]
impl crate::Resettable for PchPstateSpec {
    const RESET_VALUE: u32 = 0;
}
