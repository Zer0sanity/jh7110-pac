#[doc = "Register `comp_version` reader"]
pub type R = crate::R<CompVersionSpec>;
#[doc = "Field `comp_version` reader - "]
pub type CompVersionR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn comp_version(&self) -> CompVersionR {
        CompVersionR::new(self.bits)
    }
}
#[doc = "DesignWare I2C Compatibility Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_version::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompVersionSpec;
impl crate::RegisterSpec for CompVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_version::R`](R) reader structure"]
impl crate::Readable for CompVersionSpec {}
#[doc = "`reset()` method sets comp_version to value 0"]
impl crate::Resettable for CompVersionSpec {
    const RESET_VALUE: u32 = 0;
}
