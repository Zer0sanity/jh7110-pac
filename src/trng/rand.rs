#[doc = "Register `rand[%s]` reader"]
pub type R = crate::R<RandSpec>;
#[doc = "Field `rand` reader - TRNG random number bits"]
pub type RandR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TRNG random number bits"]
    #[inline(always)]
    pub fn rand(&self) -> RandR {
        RandR::new(self.bits)
    }
}
#[doc = "TRNG RAND Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RandSpec;
impl crate::RegisterSpec for RandSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rand::R`](R) reader structure"]
impl crate::Readable for RandSpec {}
#[doc = "`reset()` method sets rand[%s]
to value 0"]
impl crate::Resettable for RandSpec {
    const RESET_VALUE: u32 = 0;
}
