#[doc = "Register `anasts` reader"]
pub type R = crate::R<AnastsSpec>;
#[doc = "Register `anasts` writer"]
pub type W = crate::W<AnastsSpec>;
#[doc = "Field `anasts` reader - USB3 OTG ANA status."]
pub type AnastsR = crate::FieldReader<u32>;
#[doc = "Field `anasts` writer - USB3 OTG ANA status."]
pub type AnastsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USB3 OTG ANA status."]
    #[inline(always)]
    pub fn anasts(&self) -> AnastsR {
        AnastsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USB3 OTG ANA status."]
    #[inline(always)]
    #[must_use]
    pub fn anasts(&mut self) -> AnastsW<AnastsSpec> {
        AnastsW::new(self, 0)
    }
}
#[doc = "USB3 OTG ANA status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`anasts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`anasts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnastsSpec;
impl crate::RegisterSpec for AnastsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anasts::R`](R) reader structure"]
impl crate::Readable for AnastsSpec {}
#[doc = "`write(|w| ..)` method takes [`anasts::W`](W) writer structure"]
impl crate::Writable for AnastsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets anasts to value 0"]
impl crate::Resettable for AnastsSpec {
    const RESET_VALUE: u32 = 0;
}
