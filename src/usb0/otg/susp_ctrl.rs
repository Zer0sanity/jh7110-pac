#[doc = "Register `susp_ctrl` reader"]
pub type R = crate::R<SuspCtrlSpec>;
#[doc = "Register `susp_ctrl` writer"]
pub type W = crate::W<SuspCtrlSpec>;
#[doc = "Field `susp_ctrl` reader - USB3 OTG suspend control."]
pub type SuspCtrlR = crate::FieldReader<u32>;
#[doc = "Field `susp_ctrl` writer - USB3 OTG suspend control."]
pub type SuspCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USB3 OTG suspend control."]
    #[inline(always)]
    pub fn susp_ctrl(&self) -> SuspCtrlR {
        SuspCtrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USB3 OTG suspend control."]
    #[inline(always)]
    #[must_use]
    pub fn susp_ctrl(&mut self) -> SuspCtrlW<SuspCtrlSpec> {
        SuspCtrlW::new(self, 0)
    }
}
#[doc = "USB3 OTG suspend control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SuspCtrlSpec;
impl crate::RegisterSpec for SuspCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`susp_ctrl::R`](R) reader structure"]
impl crate::Readable for SuspCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`susp_ctrl::W`](W) writer structure"]
impl crate::Writable for SuspCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets susp_ctrl to value 0"]
impl crate::Resettable for SuspCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
