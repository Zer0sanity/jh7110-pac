#[doc = "Register `id` reader"]
pub type R = crate::R<IdSpec>;
#[doc = "Field `id` reader - DMAC ID value"]
pub type IdR = crate::FieldReader<u64>;
impl R {
    #[doc = "Bits 0:63 - DMAC ID value"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(self.bits)
    }
}
#[doc = "DMAC ID register contains the 64-bit identification value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdSpec;
impl crate::RegisterSpec for IdSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`id::R`](R) reader structure"]
impl crate::Readable for IdSpec {}
#[doc = "`reset()` method sets id to value 0"]
impl crate::Resettable for IdSpec {
    const RESET_VALUE: u64 = 0;
}
