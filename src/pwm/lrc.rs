#[doc = "Register `lrc` reader"]
pub type R = crate::R<LrcSpec>;
#[doc = "Register `lrc` writer"]
pub type W = crate::W<LrcSpec>;
#[doc = "Field `lrc` reader - "]
pub type LrcR = crate::FieldReader<u32>;
#[doc = "Field `lrc` writer - "]
pub type LrcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lrc(&self) -> LrcR {
        LrcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn lrc(&mut self) -> LrcW<LrcSpec> {
        LrcW::new(self, 0)
    }
}
#[doc = "Opencores PTC PWM v1 RPTC_LRC register is a 1st out of two reference/capture registers. It has two functions: - In reference mode it is used to assert low PWM output or to generate an interrupt - In capture mode it captures RPTC_CNTR value on low value of ptc_capt signal. The RPTC_LRC should have higher value than RPTC_HRC. This is because PWM output goes first high and later low.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LrcSpec;
impl crate::RegisterSpec for LrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lrc::R`](R) reader structure"]
impl crate::Readable for LrcSpec {}
#[doc = "`write(|w| ..)` method takes [`lrc::W`](W) writer structure"]
impl crate::Writable for LrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets lrc to value 0"]
impl crate::Resettable for LrcSpec {
    const RESET_VALUE: u32 = 0;
}
