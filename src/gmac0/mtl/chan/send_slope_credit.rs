#[doc = "Register `send_slope_credit` reader"]
pub type R = crate::R<SendSlopeCreditSpec>;
#[doc = "Register `send_slope_credit` writer"]
pub type W = crate::W<SendSlopeCreditSpec>;
#[doc = "Field `ssc` reader - MTL Channel Send Slope Credit"]
pub type SscR = crate::FieldReader<u16>;
#[doc = "Field `ssc` writer - MTL Channel Send Slope Credit"]
pub type SscW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - MTL Channel Send Slope Credit"]
    #[inline(always)]
    pub fn ssc(&self) -> SscR {
        SscR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - MTL Channel Send Slope Credit"]
    #[inline(always)]
    #[must_use]
    pub fn ssc(&mut self) -> SscW<SendSlopeCreditSpec> {
        SscW::new(self, 0)
    }
}
#[doc = "MTL Channel Send Slope Credit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`send_slope_credit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`send_slope_credit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SendSlopeCreditSpec;
impl crate::RegisterSpec for SendSlopeCreditSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`send_slope_credit::R`](R) reader structure"]
impl crate::Readable for SendSlopeCreditSpec {}
#[doc = "`write(|w| ..)` method takes [`send_slope_credit::W`](W) writer structure"]
impl crate::Writable for SendSlopeCreditSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets send_slope_credit to value 0"]
impl crate::Resettable for SendSlopeCreditSpec {
    const RESET_VALUE: u32 = 0;
}
