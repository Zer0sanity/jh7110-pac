#[doc = "Register `compver` reader"]
pub type R = crate::R<CompverSpec>;
#[doc = "Field `compver` reader - DMAC Component Version value"]
pub type CompverR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DMAC Component Version value"]
    #[inline(always)]
    pub fn compver(&self) -> CompverR {
        CompverR::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "DMAC Component Version register contains the 32-bit component version.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compver::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompverSpec;
impl crate::RegisterSpec for CompverSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`compver::R`](R) reader structure"]
impl crate::Readable for CompverSpec {}
#[doc = "`reset()` method sets compver to value 0"]
impl crate::Resettable for CompverSpec {
    const RESET_VALUE: u64 = 0;
}
