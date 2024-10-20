#[doc = "Register `pcmtxcr` reader"]
pub type R = crate::R<PcmtxcrSpec>;
#[doc = "Register `pcmtxcr` writer"]
pub type W = crate::W<PcmtxcrSpec>;
#[doc = "Field `tx_en` reader - TDM TX enable - 0: disable, 1: enable."]
pub type TxEnR = crate::BitReader;
#[doc = "Field `tx_en` writer - TDM TX enable - 0: disable, 1: enable."]
pub type TxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lrj` reader - TDM left-right justify - 0: right-justify, 1: left-justify."]
pub type LrjR = crate::BitReader;
#[doc = "Field `lrj` writer - TDM left-right justify - 0: right-justify, 1: left-justify."]
pub type LrjW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sl` reader - TDM slot length - 0: 8-bit, 1: 16-bit, 2: 32-bit."]
pub type SlR = crate::FieldReader;
#[doc = "Field `sl` writer - TDM slot length - 0: 8-bit, 1: 16-bit, 2: 32-bit."]
pub type SlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sscale` reader - TDM slot scale."]
pub type SscaleR = crate::FieldReader;
#[doc = "Field `sscale` writer - TDM slot scale."]
pub type SscaleW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `wl` reader - TDM word length - 0: 8-bit, 1: 16-bit, 2: 20-bit, 3: 24-bit, 4: 32-bit."]
pub type WlR = crate::FieldReader;
#[doc = "Field `wl` writer - TDM word length - 0: 8-bit, 1: 16-bit, 2: 20-bit, 3: 24-bit, 4: 32-bit."]
pub type WlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ifl` reader - TDM FIFO Length - 0: half, 1: quarter."]
pub type IflR = crate::BitReader;
#[doc = "Field `ifl` writer - TDM FIFO Length - 0: half, 1: quarter."]
pub type IflW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TDM TX enable - 0: disable, 1: enable."]
    #[inline(always)]
    pub fn tx_en(&self) -> TxEnR {
        TxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TDM left-right justify - 0: right-justify, 1: left-justify."]
    #[inline(always)]
    pub fn lrj(&self) -> LrjR {
        LrjR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - TDM slot length - 0: 8-bit, 1: 16-bit, 2: 32-bit."]
    #[inline(always)]
    pub fn sl(&self) -> SlR {
        SlR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - TDM slot scale."]
    #[inline(always)]
    pub fn sscale(&self) -> SscaleR {
        SscaleR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - TDM word length - 0: 8-bit, 1: 16-bit, 2: 20-bit, 3: 24-bit, 4: 32-bit."]
    #[inline(always)]
    pub fn wl(&self) -> WlR {
        WlR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - TDM FIFO Length - 0: half, 1: quarter."]
    #[inline(always)]
    pub fn ifl(&self) -> IflR {
        IflR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TDM TX enable - 0: disable, 1: enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_en(&mut self) -> TxEnW<PcmtxcrSpec> {
        TxEnW::new(self, 0)
    }
    #[doc = "Bit 1 - TDM left-right justify - 0: right-justify, 1: left-justify."]
    #[inline(always)]
    #[must_use]
    pub fn lrj(&mut self) -> LrjW<PcmtxcrSpec> {
        LrjW::new(self, 1)
    }
    #[doc = "Bits 2:3 - TDM slot length - 0: 8-bit, 1: 16-bit, 2: 32-bit."]
    #[inline(always)]
    #[must_use]
    pub fn sl(&mut self) -> SlW<PcmtxcrSpec> {
        SlW::new(self, 2)
    }
    #[doc = "Bits 4:7 - TDM slot scale."]
    #[inline(always)]
    #[must_use]
    pub fn sscale(&mut self) -> SscaleW<PcmtxcrSpec> {
        SscaleW::new(self, 4)
    }
    #[doc = "Bits 8:10 - TDM word length - 0: 8-bit, 1: 16-bit, 2: 20-bit, 3: 24-bit, 4: 32-bit."]
    #[inline(always)]
    #[must_use]
    pub fn wl(&mut self) -> WlW<PcmtxcrSpec> {
        WlW::new(self, 8)
    }
    #[doc = "Bit 11 - TDM FIFO Length - 0: half, 1: quarter."]
    #[inline(always)]
    #[must_use]
    pub fn ifl(&mut self) -> IflW<PcmtxcrSpec> {
        IflW::new(self, 11)
    }
}
#[doc = "TDM PCM TX Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmtxcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmtxcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcmtxcrSpec;
impl crate::RegisterSpec for PcmtxcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcmtxcr::R`](R) reader structure"]
impl crate::Readable for PcmtxcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pcmtxcr::W`](W) writer structure"]
impl crate::Writable for PcmtxcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pcmtxcr to value 0"]
impl crate::Resettable for PcmtxcrSpec {
    const RESET_VALUE: u32 = 0;
}
