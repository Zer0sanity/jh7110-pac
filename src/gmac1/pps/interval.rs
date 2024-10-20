#[doc = "Register `interval` reader"]
pub type R = crate::R<IntervalSpec>;
#[doc = "Register `interval` writer"]
pub type W = crate::W<IntervalSpec>;
#[doc = "Field `interval` reader - PPS Interval"]
pub type IntervalR = crate::FieldReader<u32>;
#[doc = "Field `interval` writer - PPS Interval"]
pub type IntervalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PPS Interval"]
    #[inline(always)]
    pub fn interval(&self) -> IntervalR {
        IntervalR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PPS Interval"]
    #[inline(always)]
    #[must_use]
    pub fn interval(&mut self) -> IntervalW<IntervalSpec> {
        IntervalW::new(self, 0)
    }
}
#[doc = "PPS Interval\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntervalSpec;
impl crate::RegisterSpec for IntervalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interval::R`](R) reader structure"]
impl crate::Readable for IntervalSpec {}
#[doc = "`write(|w| ..)` method takes [`interval::W`](W) writer structure"]
impl crate::Writable for IntervalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets interval to value 0"]
impl crate::Resettable for IntervalSpec {
    const RESET_VALUE: u32 = 0;
}
