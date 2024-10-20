#[doc = "Register `ris` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `ris` reader - Raw interrupt status from the watchdog counter."]
pub type RisR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Raw interrupt status from the watchdog counter."]
    #[inline(always)]
    pub fn ris(&self) -> RisR {
        RisR::new(self.bits)
    }
}
#[doc = "StarFive JH7110 Watchdog Raw Interrupt Status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets ris to value 0"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0;
}
