#[doc = "Register `hrc` reader"]
pub type R = crate::R<HrcSpec>;
#[doc = "Register `hrc` writer"]
pub type W = crate::W<HrcSpec>;
#[doc = "Field `hrc` reader - "]
pub type HrcR = crate::FieldReader<u32>;
#[doc = "Field `hrc` writer - "]
pub type HrcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hrc(&self) -> HrcR {
        HrcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn hrc(&mut self) -> HrcW<HrcSpec> {
        HrcW::new(self, 0)
    }
}
#[doc = "Opencores PTC PWM v1 HRC register is a 2nd out of two reference/capture registers. It has two functions: - In reference mode it is used to assert high PWM output or to generate an interrupt - In capture mode it captures RPTC_CNTR value on high value of ptc_capt signal. The RPTC_HRC should have lower value than RPTC_LRC. This is because PWM output goes first high and later low.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HrcSpec;
impl crate::RegisterSpec for HrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrc::R`](R) reader structure"]
impl crate::Readable for HrcSpec {}
#[doc = "`write(|w| ..)` method takes [`hrc::W`](W) writer structure"]
impl crate::Writable for HrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets hrc to value 0"]
impl crate::Resettable for HrcSpec {
    const RESET_VALUE: u32 = 0;
}
