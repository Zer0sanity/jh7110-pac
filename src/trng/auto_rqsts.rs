#[doc = "Register `auto_rqsts` reader"]
pub type R = crate::R<AutoRqstsSpec>;
#[doc = "Register `auto_rqsts` writer"]
pub type W = crate::W<AutoRqstsSpec>;
#[doc = "Field `rqsts` reader - Threshold number of reseed requests for auto-reseed counter"]
pub type RqstsR = crate::FieldReader<u32>;
#[doc = "Field `rqsts` writer - Threshold number of reseed requests for auto-reseed counter"]
pub type RqstsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Threshold number of reseed requests for auto-reseed counter"]
    #[inline(always)]
    pub fn rqsts(&self) -> RqstsR {
        RqstsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Threshold number of reseed requests for auto-reseed counter"]
    #[inline(always)]
    #[must_use]
    pub fn rqsts(&mut self) -> RqstsW<AutoRqstsSpec> {
        RqstsW::new(self, 0)
    }
}
#[doc = "Auto-reseeding after random number requests by host reaches specified counter: 0 - disable counter, other - reload value for internal counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_rqsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auto_rqsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AutoRqstsSpec;
impl crate::RegisterSpec for AutoRqstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auto_rqsts::R`](R) reader structure"]
impl crate::Readable for AutoRqstsSpec {}
#[doc = "`write(|w| ..)` method takes [`auto_rqsts::W`](W) writer structure"]
impl crate::Writable for AutoRqstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets auto_rqsts to value 0"]
impl crate::Resettable for AutoRqstsSpec {
    const RESET_VALUE: u32 = 0;
}
