#[doc = "Register `auto_age` reader"]
pub type R = crate::R<AutoAgeSpec>;
#[doc = "Register `auto_age` writer"]
pub type W = crate::W<AutoAgeSpec>;
#[doc = "Field `age` reader - Countdown value for auto-reseed timer"]
pub type AgeR = crate::FieldReader<u32>;
#[doc = "Field `age` writer - Countdown value for auto-reseed timer"]
pub type AgeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Countdown value for auto-reseed timer"]
    #[inline(always)]
    pub fn age(&self) -> AgeR {
        AgeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Countdown value for auto-reseed timer"]
    #[inline(always)]
    #[must_use]
    pub fn age(&mut self) -> AgeW<AutoAgeSpec> {
        AgeW::new(self, 0)
    }
}
#[doc = "Auto-reseeding after specified timer countdowns to 0: 0 - disable timer, other - reload value for internal timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_age::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auto_age::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AutoAgeSpec;
impl crate::RegisterSpec for AutoAgeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auto_age::R`](R) reader structure"]
impl crate::Readable for AutoAgeSpec {}
#[doc = "`write(|w| ..)` method takes [`auto_age::W`](W) writer structure"]
impl crate::Writable for AutoAgeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets auto_age to value 0"]
impl crate::Resettable for AutoAgeSpec {
    const RESET_VALUE: u32 = 0;
}
