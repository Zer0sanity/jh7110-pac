#[doc = "Register `count` reader"]
pub type R = crate::R<CountSpec>;
#[doc = "Field `count` reader - Reports the number of times an ECC error occured."]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reports the number of times an ECC error occured."]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "L2 Cache Control ECC Type Count register. Reports the number of times an ECC error occured.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CountSpec;
impl crate::RegisterSpec for CountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`count::R`](R) reader structure"]
impl crate::Readable for CountSpec {}
#[doc = "`reset()` method sets count to value 0"]
impl crate::Resettable for CountSpec {
    const RESET_VALUE: u32 = 0;
}
