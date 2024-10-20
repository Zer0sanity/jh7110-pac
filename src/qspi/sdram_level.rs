#[doc = "Register `sdram_level` reader"]
pub type R = crate::R<SdramLevelSpec>;
#[doc = "Field `rd` reader - SDRAM Read Level"]
pub type RdR = crate::FieldReader<u16>;
#[doc = "Field `wr` reader - SDRAM Write Level"]
pub type WrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - SDRAM Read Level"]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SDRAM Write Level"]
    #[inline(always)]
    pub fn wr(&self) -> WrR {
        WrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Cadence QSPI SDRAM Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdram_level::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdramLevelSpec;
impl crate::RegisterSpec for SdramLevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdram_level::R`](R) reader structure"]
impl crate::Readable for SdramLevelSpec {}
#[doc = "`reset()` method sets sdram_level to value 0"]
impl crate::Resettable for SdramLevelSpec {
    const RESET_VALUE: u32 = 0;
}
