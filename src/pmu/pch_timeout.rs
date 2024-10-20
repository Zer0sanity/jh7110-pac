#[doc = "Register `pch_timeout` reader"]
pub type R = crate::R<PchTimeoutSpec>;
#[doc = "Register `pch_timeout` writer"]
pub type W = crate::W<PchTimeoutSpec>;
#[doc = "Field `pch_timeout` reader - "]
pub type PchTimeoutR = crate::FieldReader;
#[doc = "Field `pch_timeout` writer - "]
pub type PchTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pch_timeout(&self) -> PchTimeoutR {
        PchTimeoutR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pch_timeout(&mut self) -> PchTimeoutW<PchTimeoutSpec> {
        PchTimeoutW::new(self, 0)
    }
}
#[doc = "P-channel waiting device acknowledge timeout.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pch_timeout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pch_timeout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PchTimeoutSpec;
impl crate::RegisterSpec for PchTimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pch_timeout::R`](R) reader structure"]
impl crate::Readable for PchTimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`pch_timeout::W`](W) writer structure"]
impl crate::Writable for PchTimeoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pch_timeout to value 0"]
impl crate::Resettable for PchTimeoutSpec {
    const RESET_VALUE: u32 = 0;
}
