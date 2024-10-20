#[doc = "Register `config` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Field `banks` reader - Number of banks in the cache."]
pub type BanksR = crate::FieldReader;
#[doc = "Field `ways` reader - Number of ways per bank."]
pub type WaysR = crate::FieldReader;
#[doc = "Field `lg_sets` reader - Base-2 logarithm of the sets per bank."]
pub type LgSetsR = crate::FieldReader;
#[doc = "Field `lg_block_bytes` reader - Base-2 logarithm of the sets per bank."]
pub type LgBlockBytesR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of banks in the cache."]
    #[inline(always)]
    pub fn banks(&self) -> BanksR {
        BanksR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of ways per bank."]
    #[inline(always)]
    pub fn ways(&self) -> WaysR {
        WaysR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Base-2 logarithm of the sets per bank."]
    #[inline(always)]
    pub fn lg_sets(&self) -> LgSetsR {
        LgSetsR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Base-2 logarithm of the sets per bank."]
    #[inline(always)]
    pub fn lg_block_bytes(&self) -> LgBlockBytesR {
        LgBlockBytesR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "L2 Cache Control configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`reset()` method sets config to value 0x060a_1002"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x060a_1002;
}
