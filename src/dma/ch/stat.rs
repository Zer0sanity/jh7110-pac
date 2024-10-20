#[doc = "Register `stat[%s]` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Field `stat` reader - Channel Status"]
pub type StatR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Status"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Channel Status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets stat[%s]
to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u64 = 0;
}
