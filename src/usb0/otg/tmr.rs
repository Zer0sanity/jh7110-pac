#[doc = "Register `tmr` reader"]
pub type R = crate::R<TmrSpec>;
#[doc = "Register `tmr` writer"]
pub type W = crate::W<TmrSpec>;
#[doc = "Field `tmr` reader - USB3 OTG timer."]
pub type TmrR = crate::FieldReader<u32>;
#[doc = "Field `tmr` writer - USB3 OTG timer."]
pub type TmrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USB3 OTG timer."]
    #[inline(always)]
    pub fn tmr(&self) -> TmrR {
        TmrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USB3 OTG timer."]
    #[inline(always)]
    #[must_use]
    pub fn tmr(&mut self) -> TmrW<TmrSpec> {
        TmrW::new(self, 0)
    }
}
#[doc = "USB3 OTG timer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmrSpec;
impl crate::RegisterSpec for TmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr::R`](R) reader structure"]
impl crate::Readable for TmrSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr::W`](W) writer structure"]
impl crate::Writable for TmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tmr to value 0"]
impl crate::Resettable for TmrSpec {
    const RESET_VALUE: u32 = 0;
}
