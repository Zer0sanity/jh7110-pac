#[doc = "Register `comp_type` reader"]
pub type R = crate::R<CompTypeSpec>;
#[doc = "Field `comp_type` reader - "]
pub type CompTypeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn comp_type(&self) -> CompTypeR {
        CompTypeR::new(self.bits)
    }
}
#[doc = "DesignWare I2C Compatibility Type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_type::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompTypeSpec;
impl crate::RegisterSpec for CompTypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_type::R`](R) reader structure"]
impl crate::Readable for CompTypeSpec {}
#[doc = "`reset()` method sets comp_type to value 0"]
impl crate::Resettable for CompTypeSpec {
    const RESET_VALUE: u32 = 0;
}
