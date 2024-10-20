#[doc = "Register `credit[%s]` reader"]
pub type R = crate::R<CreditSpec>;
#[doc = "Register `credit[%s]` writer"]
pub type W = crate::W<CreditSpec>;
#[doc = "Field `credit` reader - MTL Channel Credit"]
pub type CreditR = crate::FieldReader<u32>;
#[doc = "Field `credit` writer - MTL Channel Credit"]
pub type CreditW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - MTL Channel Credit"]
    #[inline(always)]
    pub fn credit(&self) -> CreditR {
        CreditR::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - MTL Channel Credit"]
    #[inline(always)]
    #[must_use]
    pub fn credit(&mut self) -> CreditW<CreditSpec> {
        CreditW::new(self, 0)
    }
}
#[doc = "MTL Channel Credit - credit0: High, credit1: Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`credit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`credit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CreditSpec;
impl crate::RegisterSpec for CreditSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`credit::R`](R) reader structure"]
impl crate::Readable for CreditSpec {}
#[doc = "`write(|w| ..)` method takes [`credit::W`](W) writer structure"]
impl crate::Writable for CreditSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets credit[%s]
to value 0"]
impl crate::Resettable for CreditSpec {
    const RESET_VALUE: u32 = 0;
}
