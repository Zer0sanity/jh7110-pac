#[doc = "Register `adp_ramp_time` reader"]
pub type R = crate::R<AdpRampTimeSpec>;
#[doc = "Register `adp_ramp_time` writer"]
pub type W = crate::W<AdpRampTimeSpec>;
#[doc = "Field `adp_ramp_time` reader - USB3 OTG ADP ramp time."]
pub type AdpRampTimeR = crate::FieldReader<u32>;
#[doc = "Field `adp_ramp_time` writer - USB3 OTG ADP ramp time."]
pub type AdpRampTimeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USB3 OTG ADP ramp time."]
    #[inline(always)]
    pub fn adp_ramp_time(&self) -> AdpRampTimeR {
        AdpRampTimeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USB3 OTG ADP ramp time."]
    #[inline(always)]
    #[must_use]
    pub fn adp_ramp_time(&mut self) -> AdpRampTimeW<AdpRampTimeSpec> {
        AdpRampTimeW::new(self, 0)
    }
}
#[doc = "USB3 OTG ADP ramp time.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adp_ramp_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adp_ramp_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdpRampTimeSpec;
impl crate::RegisterSpec for AdpRampTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adp_ramp_time::R`](R) reader structure"]
impl crate::Readable for AdpRampTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`adp_ramp_time::W`](W) writer structure"]
impl crate::Writable for AdpRampTimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets adp_ramp_time to value 0"]
impl crate::Resettable for AdpRampTimeSpec {
    const RESET_VALUE: u32 = 0;
}
