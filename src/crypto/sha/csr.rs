#[doc = "Register `csr` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `csr` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `start` reader - SHA CSR Start"]
pub type StartR = crate::BitReader;
#[doc = "Field `start` writer - SHA CSR Start"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reset` reader - SHA Reset"]
pub type ResetR = crate::BitReader;
#[doc = "Field `reset` writer - SHA Reset"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ie` reader - SHA Interrupt Enable"]
pub type IeR = crate::BitReader;
#[doc = "Field `ie` writer - SHA Interrupt Enable"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `firstb` reader - SHA First B"]
pub type FirstbR = crate::BitReader;
#[doc = "Field `firstb` writer - SHA First B"]
pub type FirstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mode` reader - SHA Mode - 0: SM3, 1: SHA0, 2: SHA1, 3: SHA224, 4: SHA256, 5: SHA384, 6: SHA512, 7: MASK"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `mode` writer - SHA Mode - 0: SM3, 1: SHA0, 2: SHA1, 3: SHA224, 4: SHA256, 5: SHA384, 6: SHA512, 7: MASK"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sha_final` reader - SHA Final"]
pub type ShaFinalR = crate::BitReader;
#[doc = "Field `sha_final` writer - SHA Final"]
pub type ShaFinalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hmac` reader - SHA HMAC"]
pub type HmacR = crate::BitReader;
#[doc = "Field `hmac` writer - SHA HMAC"]
pub type HmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `key_done` reader - SHA Key Done"]
pub type KeyDoneR = crate::BitReader;
#[doc = "Field `key_done` writer - SHA Key Done"]
pub type KeyDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `key_flag` reader - SHA Key Flag"]
pub type KeyFlagR = crate::BitReader;
#[doc = "Field `key_flag` writer - SHA Key Flag"]
pub type KeyFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hmac_done` reader - SHA HMAC Done"]
pub type HmacDoneR = crate::BitReader;
#[doc = "Field `hmac_done` writer - SHA HMAC Done"]
pub type HmacDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `busy` reader - SHA Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `busy` writer - SHA Busy"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `shadone` reader - SHA Done"]
pub type ShadoneR = crate::BitReader;
#[doc = "Field `shadone` writer - SHA Done"]
pub type ShadoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SHA CSR Start"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SHA Reset"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SHA Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SHA First B"]
    #[inline(always)]
    pub fn firstb(&self) -> FirstbR {
        FirstbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SHA Mode - 0: SM3, 1: SHA0, 2: SHA1, 3: SHA224, 4: SHA256, 5: SHA384, 6: SHA512, 7: MASK"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - SHA Final"]
    #[inline(always)]
    pub fn sha_final(&self) -> ShaFinalR {
        ShaFinalR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - SHA HMAC"]
    #[inline(always)]
    pub fn hmac(&self) -> HmacR {
        HmacR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - SHA Key Done"]
    #[inline(always)]
    pub fn key_done(&self) -> KeyDoneR {
        KeyDoneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SHA Key Flag"]
    #[inline(always)]
    pub fn key_flag(&self) -> KeyFlagR {
        KeyFlagR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SHA HMAC Done"]
    #[inline(always)]
    pub fn hmac_done(&self) -> HmacDoneR {
        HmacDoneR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SHA Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SHA Done"]
    #[inline(always)]
    pub fn shadone(&self) -> ShadoneR {
        ShadoneR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SHA CSR Start"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<CsrSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - SHA Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<CsrSpec> {
        ResetW::new(self, 1)
    }
    #[doc = "Bit 2 - SHA Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IeW<CsrSpec> {
        IeW::new(self, 2)
    }
    #[doc = "Bit 3 - SHA First B"]
    #[inline(always)]
    #[must_use]
    pub fn firstb(&mut self) -> FirstbW<CsrSpec> {
        FirstbW::new(self, 3)
    }
    #[doc = "Bits 4:6 - SHA Mode - 0: SM3, 1: SHA0, 2: SHA1, 3: SHA224, 4: SHA256, 5: SHA384, 6: SHA512, 7: MASK"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CsrSpec> {
        ModeW::new(self, 4)
    }
    #[doc = "Bit 8 - SHA Final"]
    #[inline(always)]
    #[must_use]
    pub fn sha_final(&mut self) -> ShaFinalW<CsrSpec> {
        ShaFinalW::new(self, 8)
    }
    #[doc = "Bit 11 - SHA HMAC"]
    #[inline(always)]
    #[must_use]
    pub fn hmac(&mut self) -> HmacW<CsrSpec> {
        HmacW::new(self, 11)
    }
    #[doc = "Bit 13 - SHA Key Done"]
    #[inline(always)]
    #[must_use]
    pub fn key_done(&mut self) -> KeyDoneW<CsrSpec> {
        KeyDoneW::new(self, 13)
    }
    #[doc = "Bit 14 - SHA Key Flag"]
    #[inline(always)]
    #[must_use]
    pub fn key_flag(&mut self) -> KeyFlagW<CsrSpec> {
        KeyFlagW::new(self, 14)
    }
    #[doc = "Bit 15 - SHA HMAC Done"]
    #[inline(always)]
    #[must_use]
    pub fn hmac_done(&mut self) -> HmacDoneW<CsrSpec> {
        HmacDoneW::new(self, 15)
    }
    #[doc = "Bit 16 - SHA Busy"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<CsrSpec> {
        BusyW::new(self, 16)
    }
    #[doc = "Bit 17 - SHA Done"]
    #[inline(always)]
    #[must_use]
    pub fn shadone(&mut self) -> ShadoneW<CsrSpec> {
        ShadoneW::new(self, 17)
    }
}
#[doc = "JH7110 Crypto SHA CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets csr to value 0"]
impl crate::Resettable for CsrSpec {
    const RESET_VALUE: u32 = 0;
}
