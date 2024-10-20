#[doc = "Register `value` reader"]
pub type R = crate::R<ValueSpec>;
#[doc = "Field `load` reader - Current value for the watchdog counter."]
pub type LoadR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current value for the watchdog counter."]
    #[inline(always)]
    pub fn load(&self) -> LoadR {
        LoadR::new(self.bits)
    }
}
#[doc = "StarFive JH7110 Watchdog Value register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`value::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for ValueSpec {}
#[doc = "`reset()` method sets value to value 0"]
impl crate::Resettable for ValueSpec {
    const RESET_VALUE: u32 = 0;
}
