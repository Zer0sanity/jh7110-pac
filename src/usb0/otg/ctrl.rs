#[doc = "Register `ctrl[%s]` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `ctrl[%s]` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ctrl` reader - USB3 OTG control."]
pub type CtrlR = crate::FieldReader<u32>;
#[doc = "Field `ctrl` writer - USB3 OTG control."]
pub type CtrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USB3 OTG control."]
    #[inline(always)]
    pub fn ctrl(&self) -> CtrlR {
        CtrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USB3 OTG control."]
    #[inline(always)]
    #[must_use]
    pub fn ctrl(&mut self) -> CtrlW<CtrlSpec> {
        CtrlW::new(self, 0)
    }
}
#[doc = "USB3 OTG control registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ctrl[%s]
to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
