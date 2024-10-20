#[doc = "Register `tbi` reader"]
pub type R = crate::R<TbiSpec>;
#[doc = "Register `tbi` writer"]
pub type W = crate::W<TbiSpec>;
#[doc = "Field `tbi` reader - TBI Extend Status"]
pub type TbiR = crate::FieldReader<u32>;
#[doc = "Field `tbi` writer - TBI Extend Status"]
pub type TbiW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TBI Extend Status"]
    #[inline(always)]
    pub fn tbi(&self) -> TbiR {
        TbiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TBI Extend Status"]
    #[inline(always)]
    #[must_use]
    pub fn tbi(&mut self) -> TbiW<TbiSpec> {
        TbiW::new(self, 0)
    }
}
#[doc = "TBI Extend Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbiSpec;
impl crate::RegisterSpec for TbiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbi::R`](R) reader structure"]
impl crate::Readable for TbiSpec {}
#[doc = "`write(|w| ..)` method takes [`tbi::W`](W) writer structure"]
impl crate::Writable for TbiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tbi to value 0"]
impl crate::Resettable for TbiSpec {
    const RESET_VALUE: u32 = 0;
}
