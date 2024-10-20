#[doc = "Register `pch_active` reader"]
pub type R = crate::R<PchActiveSpec>;
#[doc = "Field `pch_active` reader - "]
pub type PchActiveR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn pch_active(&self) -> PchActiveR {
        PchActiveR::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "P-channel PACTIVE Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pch_active::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PchActiveSpec;
impl crate::RegisterSpec for PchActiveSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pch_active::R`](R) reader structure"]
impl crate::Readable for PchActiveSpec {}
#[doc = "`reset()` method sets pch_active to value 0"]
impl crate::Resettable for PchActiveSpec {
    const RESET_VALUE: u32 = 0;
}
