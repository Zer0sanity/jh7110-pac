#[doc = "Register `csr` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `csr` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `cmode` reader - "]
pub type CmodeR = crate::BitReader;
#[doc = "Field `cmode` writer - "]
pub type CmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `keymode` reader - AES Key Mode - 0: AES 128, 1: AES 192, 2: AES 256"]
pub type KeymodeR = crate::FieldReader;
#[doc = "Field `keymode` writer - AES Key Mode - 0: AES 128, 1: AES 192, 2: AES 256"]
pub type KeymodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `busy` reader - AES Engine Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `busy` writer - AES Engine Busy"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `done` reader - AES Engine Done"]
pub type DoneR = crate::BitReader;
#[doc = "Field `done` writer - AES Engine Done"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `krdy` reader - AES Key Done"]
pub type KrdyR = crate::BitReader;
#[doc = "Field `krdy` writer - AES Key Done"]
pub type KrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst` reader - AES Reset"]
pub type RstR = crate::BitReader;
#[doc = "Field `rst` writer - AES Reset"]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ie` reader - AES Interrupt Enable"]
pub type IeR = crate::BitReader;
#[doc = "Field `ie` writer - AES Interrupt Enable"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ccm_start` reader - AES CCM Start"]
pub type CcmStartR = crate::BitReader;
#[doc = "Field `ccm_start` writer - AES CCM Start"]
pub type CcmStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mode` reader - AES Mode - 0: ECB, 1: CBC, 2: CFB, 3: OFB, 4: CTR, 5: CCM, 6: GCM"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `mode` writer - AES Mode - 0: ECB, 1: CBC, 2: CFB, 3: OFB, 4: CTR, 5: CCM, 6: GCM"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gcm_start` reader - AES GCM Start"]
pub type GcmStartR = crate::BitReader;
#[doc = "Field `gcm_start` writer - AES GCM Start"]
pub type GcmStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gcm_done` reader - AES GCM Done"]
pub type GcmDoneR = crate::BitReader;
#[doc = "Field `gcm_done` writer - AES GCM Done"]
pub type GcmDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `delay_aes` reader - Delay AES"]
pub type DelayAesR = crate::BitReader;
#[doc = "Field `delay_aes` writer - Delay AES"]
pub type DelayAesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vaes_start` reader - VAES Start"]
pub type VaesStartR = crate::BitReader;
#[doc = "Field `vaes_start` writer - VAES Start"]
pub type VaesStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stream_mode` reader - AES Stream Cipher mode - 0: XFB 1, 5: XFB 128"]
pub type StreamModeR = crate::FieldReader;
#[doc = "Field `stream_mode` writer - AES Stream Cipher mode - 0: XFB 1, 5: XFB 128"]
pub type StreamModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmode(&self) -> CmodeR {
        CmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - AES Key Mode - 0: AES 128, 1: AES 192, 2: AES 256"]
    #[inline(always)]
    pub fn keymode(&self) -> KeymodeR {
        KeymodeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - AES Engine Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AES Engine Done"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AES Key Done"]
    #[inline(always)]
    pub fn krdy(&self) -> KrdyR {
        KrdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AES Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AES Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AES CCM Start"]
    #[inline(always)]
    pub fn ccm_start(&self) -> CcmStartR {
        CcmStartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - AES Mode - 0: ECB, 1: CBC, 2: CFB, 3: OFB, 4: CTR, 5: CCM, 6: GCM"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - AES GCM Start"]
    #[inline(always)]
    pub fn gcm_start(&self) -> GcmStartR {
        GcmStartR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AES GCM Done"]
    #[inline(always)]
    pub fn gcm_done(&self) -> GcmDoneR {
        GcmDoneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Delay AES"]
    #[inline(always)]
    pub fn delay_aes(&self) -> DelayAesR {
        DelayAesR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VAES Start"]
    #[inline(always)]
    pub fn vaes_start(&self) -> VaesStartR {
        VaesStartR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 24:26 - AES Stream Cipher mode - 0: XFB 1, 5: XFB 128"]
    #[inline(always)]
    pub fn stream_mode(&self) -> StreamModeR {
        StreamModeR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cmode(&mut self) -> CmodeW<CsrSpec> {
        CmodeW::new(self, 0)
    }
    #[doc = "Bits 1:2 - AES Key Mode - 0: AES 128, 1: AES 192, 2: AES 256"]
    #[inline(always)]
    #[must_use]
    pub fn keymode(&mut self) -> KeymodeW<CsrSpec> {
        KeymodeW::new(self, 1)
    }
    #[doc = "Bit 3 - AES Engine Busy"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<CsrSpec> {
        BusyW::new(self, 3)
    }
    #[doc = "Bit 4 - AES Engine Done"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<CsrSpec> {
        DoneW::new(self, 4)
    }
    #[doc = "Bit 5 - AES Key Done"]
    #[inline(always)]
    #[must_use]
    pub fn krdy(&mut self) -> KrdyW<CsrSpec> {
        KrdyW::new(self, 5)
    }
    #[doc = "Bit 6 - AES Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RstW<CsrSpec> {
        RstW::new(self, 6)
    }
    #[doc = "Bit 7 - AES Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IeW<CsrSpec> {
        IeW::new(self, 7)
    }
    #[doc = "Bit 8 - AES CCM Start"]
    #[inline(always)]
    #[must_use]
    pub fn ccm_start(&mut self) -> CcmStartW<CsrSpec> {
        CcmStartW::new(self, 8)
    }
    #[doc = "Bits 9:11 - AES Mode - 0: ECB, 1: CBC, 2: CFB, 3: OFB, 4: CTR, 5: CCM, 6: GCM"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CsrSpec> {
        ModeW::new(self, 9)
    }
    #[doc = "Bit 12 - AES GCM Start"]
    #[inline(always)]
    #[must_use]
    pub fn gcm_start(&mut self) -> GcmStartW<CsrSpec> {
        GcmStartW::new(self, 12)
    }
    #[doc = "Bit 13 - AES GCM Done"]
    #[inline(always)]
    #[must_use]
    pub fn gcm_done(&mut self) -> GcmDoneW<CsrSpec> {
        GcmDoneW::new(self, 13)
    }
    #[doc = "Bit 14 - Delay AES"]
    #[inline(always)]
    #[must_use]
    pub fn delay_aes(&mut self) -> DelayAesW<CsrSpec> {
        DelayAesW::new(self, 14)
    }
    #[doc = "Bit 15 - VAES Start"]
    #[inline(always)]
    #[must_use]
    pub fn vaes_start(&mut self) -> VaesStartW<CsrSpec> {
        VaesStartW::new(self, 15)
    }
    #[doc = "Bits 24:26 - AES Stream Cipher mode - 0: XFB 1, 5: XFB 128"]
    #[inline(always)]
    #[must_use]
    pub fn stream_mode(&mut self) -> StreamModeW<CsrSpec> {
        StreamModeW::new(self, 24)
    }
}
#[doc = "JH7110 Crypto AES Control Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
