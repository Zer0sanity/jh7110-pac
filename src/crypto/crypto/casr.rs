#[doc = "Register `casr` reader"]
pub type R = crate::R<CasrSpec>;
#[doc = "Field `done` reader - Crypto Done"]
pub type DoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Crypto Done"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
}
#[doc = "JH7110 Crypto CA Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`casr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CasrSpec;
impl crate::RegisterSpec for CasrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`casr::R`](R) reader structure"]
impl crate::Readable for CasrSpec {}
#[doc = "`reset()` method sets casr to value 0"]
impl crate::Resettable for CasrSpec {
    const RESET_VALUE: u32 = 0;
}
