#[doc = "Register `flag` reader"]
pub type R = crate::R<FlagSpec>;
#[doc = "Register `flag` writer"]
pub type W = crate::W<FlagSpec>;
#[doc = "Field `aes_ie_done` reader - AES Interrupt Enable Done"]
pub type AesIeDoneR = crate::BitReader;
#[doc = "Field `aes_ie_done` writer - AES Interrupt Enable Done"]
pub type AesIeDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `des_ie_done` reader - DES Interrupt Enable Done"]
pub type DesIeDoneR = crate::BitReader;
#[doc = "Field `des_ie_done` writer - DES Interrupt Enable Done"]
pub type DesIeDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sha_ie_done` reader - SHA Interrupt Enable Done"]
pub type ShaIeDoneR = crate::BitReader;
#[doc = "Field `sha_ie_done` writer - SHA Interrupt Enable Done"]
pub type ShaIeDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `crypto_ie_done` reader - Crypto Interrupt Enable Done"]
pub type CryptoIeDoneR = crate::BitReader;
#[doc = "Field `crypto_ie_done` writer - Crypto Interrupt Enable Done"]
pub type CryptoIeDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AES Interrupt Enable Done"]
    #[inline(always)]
    pub fn aes_ie_done(&self) -> AesIeDoneR {
        AesIeDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DES Interrupt Enable Done"]
    #[inline(always)]
    pub fn des_ie_done(&self) -> DesIeDoneR {
        DesIeDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SHA Interrupt Enable Done"]
    #[inline(always)]
    pub fn sha_ie_done(&self) -> ShaIeDoneR {
        ShaIeDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Crypto Interrupt Enable Done"]
    #[inline(always)]
    pub fn crypto_ie_done(&self) -> CryptoIeDoneR {
        CryptoIeDoneR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES Interrupt Enable Done"]
    #[inline(always)]
    #[must_use]
    pub fn aes_ie_done(&mut self) -> AesIeDoneW<FlagSpec> {
        AesIeDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - DES Interrupt Enable Done"]
    #[inline(always)]
    #[must_use]
    pub fn des_ie_done(&mut self) -> DesIeDoneW<FlagSpec> {
        DesIeDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - SHA Interrupt Enable Done"]
    #[inline(always)]
    #[must_use]
    pub fn sha_ie_done(&mut self) -> ShaIeDoneW<FlagSpec> {
        ShaIeDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - Crypto Interrupt Enable Done"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_ie_done(&mut self) -> CryptoIeDoneW<FlagSpec> {
        CryptoIeDoneW::new(self, 3)
    }
}
#[doc = "JH7110 Crypto Interrupt Enable Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlagSpec;
impl crate::RegisterSpec for FlagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flag::R`](R) reader structure"]
impl crate::Readable for FlagSpec {}
#[doc = "`write(|w| ..)` method takes [`flag::W`](W) writer structure"]
impl crate::Writable for FlagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets flag to value 0"]
impl crate::Resettable for FlagSpec {
    const RESET_VALUE: u32 = 0;
}
