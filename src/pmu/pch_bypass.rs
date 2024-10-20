#[doc = "Register `pch_bypass` reader"]
pub type R = crate::R<PchBypassSpec>;
#[doc = "Register `pch_bypass` writer"]
pub type W = crate::W<PchBypassSpec>;
#[doc = "Field `pch_bypass` reader - Bypass P-channel. 0: enable p-channel, 1: bypass p-channel"]
pub type PchBypassR = crate::BitReader;
#[doc = "Field `pch_bypass` writer - Bypass P-channel. 0: enable p-channel, 1: bypass p-channel"]
pub type PchBypassW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bypass P-channel. 0: enable p-channel, 1: bypass p-channel"]
    #[inline(always)]
    pub fn pch_bypass(&self) -> PchBypassR {
        PchBypassR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass P-channel. 0: enable p-channel, 1: bypass p-channel"]
    #[inline(always)]
    #[must_use]
    pub fn pch_bypass(&mut self) -> PchBypassW<PchBypassSpec> {
        PchBypassW::new(self, 0)
    }
}
#[doc = "P-channel Bypass register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pch_bypass::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pch_bypass::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PchBypassSpec;
impl crate::RegisterSpec for PchBypassSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pch_bypass::R`](R) reader structure"]
impl crate::Readable for PchBypassSpec {}
#[doc = "`write(|w| ..)` method takes [`pch_bypass::W`](W) writer structure"]
impl crate::Writable for PchBypassSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pch_bypass to value 0"]
impl crate::Resettable for PchBypassSpec {
    const RESET_VALUE: u32 = 0;
}
