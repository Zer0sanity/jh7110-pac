#[doc = "Register `addr[%s]` reader"]
pub type R = crate::R<AddrSpec>;
#[doc = "Field `addr` reader - ECC type most recent address to fail."]
pub type AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC type most recent address to fail."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
#[doc = "L2 Cache Control ECC Type Address registers. Contains the low- and high-address bits of the most recent failure.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrSpec;
impl crate::RegisterSpec for AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for AddrSpec {}
#[doc = "`reset()` method sets addr[%s]
to value 0"]
impl crate::Resettable for AddrSpec {
    const RESET_VALUE: u32 = 0;
}
