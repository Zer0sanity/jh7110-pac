#[doc = "Register `statar[%s]` reader"]
pub type R = crate::R<StatarSpec>;
#[doc = "Field `statar` reader - Channel Status"]
pub type StatarR = crate::FieldReader<u64>;
impl R {
    #[doc = "Bits 0:63 - Channel Status"]
    #[inline(always)]
    pub fn statar(&self) -> StatarR {
        StatarR::new(self.bits)
    }
}
#[doc = "Channel Status Fetch Address register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatarSpec;
impl crate::RegisterSpec for StatarSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`statar::R`](R) reader structure"]
impl crate::Readable for StatarSpec {}
#[doc = "`reset()` method sets statar[%s]
to value 0"]
impl crate::Resettable for StatarSpec {
    const RESET_VALUE: u64 = 0;
}
