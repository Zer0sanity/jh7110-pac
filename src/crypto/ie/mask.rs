#[doc = "Register `mask` reader"]
pub type R = crate::R<MaskSpec>;
#[doc = "Register `mask` writer"]
pub type W = crate::W<MaskSpec>;
#[doc = "Field `aes_ie_mask` reader - AES Interrupt Enable Mask"]
pub type AesIeMaskR = crate::BitReader;
#[doc = "Field `aes_ie_mask` writer - AES Interrupt Enable Mask"]
pub type AesIeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `des_ie_mask` reader - DES Interrupt Enable Mask"]
pub type DesIeMaskR = crate::BitReader;
#[doc = "Field `des_ie_mask` writer - DES Interrupt Enable Mask"]
pub type DesIeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sha_ie_mask` reader - SHA Interrupt Enable Mask"]
pub type ShaIeMaskR = crate::BitReader;
#[doc = "Field `sha_ie_mask` writer - SHA Interrupt Enable Mask"]
pub type ShaIeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `crypto_ie_mask` reader - Crypto Interrupt Enable Mask"]
pub type CryptoIeMaskR = crate::BitReader;
#[doc = "Field `crypto_ie_mask` writer - Crypto Interrupt Enable Mask"]
pub type CryptoIeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AES Interrupt Enable Mask"]
    #[inline(always)]
    pub fn aes_ie_mask(&self) -> AesIeMaskR {
        AesIeMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DES Interrupt Enable Mask"]
    #[inline(always)]
    pub fn des_ie_mask(&self) -> DesIeMaskR {
        DesIeMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SHA Interrupt Enable Mask"]
    #[inline(always)]
    pub fn sha_ie_mask(&self) -> ShaIeMaskR {
        ShaIeMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Crypto Interrupt Enable Mask"]
    #[inline(always)]
    pub fn crypto_ie_mask(&self) -> CryptoIeMaskR {
        CryptoIeMaskR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES Interrupt Enable Mask"]
    #[inline(always)]
    #[must_use]
    pub fn aes_ie_mask(&mut self) -> AesIeMaskW<MaskSpec> {
        AesIeMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - DES Interrupt Enable Mask"]
    #[inline(always)]
    #[must_use]
    pub fn des_ie_mask(&mut self) -> DesIeMaskW<MaskSpec> {
        DesIeMaskW::new(self, 1)
    }
    #[doc = "Bit 2 - SHA Interrupt Enable Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sha_ie_mask(&mut self) -> ShaIeMaskW<MaskSpec> {
        ShaIeMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - Crypto Interrupt Enable Mask"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_ie_mask(&mut self) -> CryptoIeMaskW<MaskSpec> {
        CryptoIeMaskW::new(self, 3)
    }
}
#[doc = "JH7110 Crypto Interrupt Enable Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskSpec;
impl crate::RegisterSpec for MaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask::R`](R) reader structure"]
impl crate::Readable for MaskSpec {}
#[doc = "`write(|w| ..)` method takes [`mask::W`](W) writer structure"]
impl crate::Writable for MaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mask to value 0"]
impl crate::Resettable for MaskSpec {
    const RESET_VALUE: u32 = 0;
}
