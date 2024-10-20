#[doc = "Register `an_ctrl` reader"]
pub type R = crate::R<AnCtrlSpec>;
#[doc = "Register `an_ctrl` writer"]
pub type W = crate::W<AnCtrlSpec>;
#[doc = "Field `ran` reader - Restart Auto-Negotiation"]
pub type RanR = crate::BitReader;
#[doc = "Field `ran` writer - Restart Auto-Negotiation"]
pub type RanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ane` reader - Auto-Negotiation Enable"]
pub type AneR = crate::BitReader;
#[doc = "Field `ane` writer - Auto-Negotiation Enable"]
pub type AneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ele` reader - External Loopback Enable"]
pub type EleR = crate::BitReader;
#[doc = "Field `ele` writer - External Loopback Enable"]
pub type EleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ecd` reader - Enable Comma Detect"]
pub type EcdR = crate::BitReader;
#[doc = "Field `ecd` writer - Enable Comma Detect"]
pub type EcdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lr` reader - Lock to Reference"]
pub type LrR = crate::BitReader;
#[doc = "Field `lr` writer - Lock to Reference"]
pub type LrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sgmral` reader - SGMII RAL Control"]
pub type SgmralR = crate::BitReader;
#[doc = "Field `sgmral` writer - SGMII RAL Control"]
pub type SgmralW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 9 - Restart Auto-Negotiation"]
    #[inline(always)]
    pub fn ran(&self) -> RanR {
        RanR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Auto-Negotiation Enable"]
    #[inline(always)]
    pub fn ane(&self) -> AneR {
        AneR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - External Loopback Enable"]
    #[inline(always)]
    pub fn ele(&self) -> EleR {
        EleR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Comma Detect"]
    #[inline(always)]
    pub fn ecd(&self) -> EcdR {
        EcdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Lock to Reference"]
    #[inline(always)]
    pub fn lr(&self) -> LrR {
        LrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SGMII RAL Control"]
    #[inline(always)]
    pub fn sgmral(&self) -> SgmralR {
        SgmralR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Restart Auto-Negotiation"]
    #[inline(always)]
    #[must_use]
    pub fn ran(&mut self) -> RanW<AnCtrlSpec> {
        RanW::new(self, 9)
    }
    #[doc = "Bit 12 - Auto-Negotiation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ane(&mut self) -> AneW<AnCtrlSpec> {
        AneW::new(self, 12)
    }
    #[doc = "Bit 14 - External Loopback Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ele(&mut self) -> EleW<AnCtrlSpec> {
        EleW::new(self, 14)
    }
    #[doc = "Bit 16 - Enable Comma Detect"]
    #[inline(always)]
    #[must_use]
    pub fn ecd(&mut self) -> EcdW<AnCtrlSpec> {
        EcdW::new(self, 16)
    }
    #[doc = "Bit 17 - Lock to Reference"]
    #[inline(always)]
    #[must_use]
    pub fn lr(&mut self) -> LrW<AnCtrlSpec> {
        LrW::new(self, 17)
    }
    #[doc = "Bit 18 - SGMII RAL Control"]
    #[inline(always)]
    #[must_use]
    pub fn sgmral(&mut self) -> SgmralW<AnCtrlSpec> {
        SgmralW::new(self, 18)
    }
}
#[doc = "Auto-Negotiation Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`an_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`an_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnCtrlSpec;
impl crate::RegisterSpec for AnCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`an_ctrl::R`](R) reader structure"]
impl crate::Readable for AnCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`an_ctrl::W`](W) writer structure"]
impl crate::Writable for AnCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets an_ctrl to value 0"]
impl crate::Resettable for AnCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
