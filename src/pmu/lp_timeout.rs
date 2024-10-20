#[doc = "Register `lp_timeout` reader"]
pub type R = crate::R<LpTimeoutSpec>;
#[doc = "Register `lp_timeout` writer"]
pub type W = crate::W<LpTimeoutSpec>;
#[doc = "Field `lp_timeout` reader - "]
pub type LpTimeoutR = crate::FieldReader;
#[doc = "Field `lp_timeout` writer - "]
pub type LpTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn lp_timeout(&self) -> LpTimeoutR {
        LpTimeoutR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn lp_timeout(&mut self) -> LpTimeoutW<LpTimeoutSpec> {
        LpTimeoutW::new(self, 0)
    }
}
#[doc = "LP Cell Control Timeout Threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_timeout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_timeout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpTimeoutSpec;
impl crate::RegisterSpec for LpTimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_timeout::R`](R) reader structure"]
impl crate::Readable for LpTimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_timeout::W`](W) writer structure"]
impl crate::Writable for LpTimeoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets lp_timeout to value 0"]
impl crate::Resettable for LpTimeoutSpec {
    const RESET_VALUE: u32 = 0;
}
