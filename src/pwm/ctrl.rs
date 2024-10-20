#[doc = "Register `ctrl` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `ctrl` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `en` reader - When set, RPTC_CNTR can be incremented."]
pub type EnR = crate::BitReader;
#[doc = "Field `en` writer - When set, RPTC_CNTR can be incremented."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `eclk` reader - When set, ptc_ecgt signal is used to increment RPTC_CNTR. When cleared, system clock is used instead."]
pub type EclkR = crate::BitReader;
#[doc = "Field `eclk` writer - When set, ptc_ecgt signal is used to increment RPTC_CNTR. When cleared, system clock is used instead."]
pub type EclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nec` reader - When set, ptc_ecgt increments on negative edge and gates on low period. When cleared, ptc_ecgt increments on positive edge and gates on high period. This bit has effect only on `gating` function of ptc_ecgt when RPTC_CTRL\\[ECLK\\]
bit is cleared."]
pub type NecR = crate::BitReader;
#[doc = "Field `nec` writer - When set, ptc_ecgt increments on negative edge and gates on low period. When cleared, ptc_ecgt increments on positive edge and gates on high period. This bit has effect only on `gating` function of ptc_ecgt when RPTC_CTRL\\[ECLK\\]
bit is cleared."]
pub type NecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `oe` reader - Inverted value of this bit is reflected on the ptc_oen signal. It is used to enable PWM output driver."]
pub type OeR = crate::BitReader;
#[doc = "Field `oe` writer - Inverted value of this bit is reflected on the ptc_oen signal. It is used to enable PWM output driver."]
pub type OeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `single` reader - When set, RPTC_CNTR is not incremented anymore after it reaches value equal to the RPTC_LRC value. When cleared, RPTC_CNTR is restarted after it reaches value in the RPTC_LCR register."]
pub type SingleR = crate::BitReader;
#[doc = "Field `single` writer - When set, RPTC_CNTR is not incremented anymore after it reaches value equal to the RPTC_LRC value. When cleared, RPTC_CNTR is restarted after it reaches value in the RPTC_LCR register."]
pub type SingleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `inte` reader - When set, PTC asserts an interrupt when RPTC_CNTR value is equal to the value of RPTC_LRC or RPTC_HRC. When cleared, interrupts are masked."]
pub type InteR = crate::BitReader;
#[doc = "Field `inte` writer - When set, PTC asserts an interrupt when RPTC_CNTR value is equal to the value of RPTC_LRC or RPTC_HRC. When cleared, interrupts are masked."]
pub type InteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `int` reader - When read, this bit represents pending interrupt. When it is set, an interrupt is pending. When this bit is written with `1`, interrupt request is cleared."]
pub type IntR = crate::BitReader;
#[doc = "Field `int` writer - When read, this bit represents pending interrupt. When it is set, an interrupt is pending. When this bit is written with `1`, interrupt request is cleared."]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cntrrst` reader - When set, RPTC_CNTR is under reset. When cleared, normal operation of the counter is allowed."]
pub type CntrrstR = crate::BitReader;
#[doc = "Field `cntrrst` writer - When set, RPTC_CNTR is under reset. When cleared, normal operation of the counter is allowed."]
pub type CntrrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `capte` reader - When set, ptc_capt signal can be used to capture RPTC_CNTR into RPTC_LRC or RPTC_HRC registers. Into which reference/capture register capture occurs depends on edge of the ptc_capt signal. When cleared, capture function is masked."]
pub type CapteR = crate::BitReader;
#[doc = "Field `capte` writer - When set, ptc_capt signal can be used to capture RPTC_CNTR into RPTC_LRC or RPTC_HRC registers. Into which reference/capture register capture occurs depends on edge of the ptc_capt signal. When cleared, capture function is masked."]
pub type CapteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set, RPTC_CNTR can be incremented."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set, ptc_ecgt signal is used to increment RPTC_CNTR. When cleared, system clock is used instead."]
    #[inline(always)]
    pub fn eclk(&self) -> EclkR {
        EclkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set, ptc_ecgt increments on negative edge and gates on low period. When cleared, ptc_ecgt increments on positive edge and gates on high period. This bit has effect only on `gating` function of ptc_ecgt when RPTC_CTRL\\[ECLK\\]
bit is cleared."]
    #[inline(always)]
    pub fn nec(&self) -> NecR {
        NecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Inverted value of this bit is reflected on the ptc_oen signal. It is used to enable PWM output driver."]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When set, RPTC_CNTR is not incremented anymore after it reaches value equal to the RPTC_LRC value. When cleared, RPTC_CNTR is restarted after it reaches value in the RPTC_LCR register."]
    #[inline(always)]
    pub fn single(&self) -> SingleR {
        SingleR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When set, PTC asserts an interrupt when RPTC_CNTR value is equal to the value of RPTC_LRC or RPTC_HRC. When cleared, interrupts are masked."]
    #[inline(always)]
    pub fn inte(&self) -> InteR {
        InteR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When read, this bit represents pending interrupt. When it is set, an interrupt is pending. When this bit is written with `1`, interrupt request is cleared."]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When set, RPTC_CNTR is under reset. When cleared, normal operation of the counter is allowed."]
    #[inline(always)]
    pub fn cntrrst(&self) -> CntrrstR {
        CntrrstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - When set, ptc_capt signal can be used to capture RPTC_CNTR into RPTC_LRC or RPTC_HRC registers. Into which reference/capture register capture occurs depends on edge of the ptc_capt signal. When cleared, capture function is masked."]
    #[inline(always)]
    pub fn capte(&self) -> CapteR {
        CapteR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set, RPTC_CNTR can be incremented."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<CtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - When set, ptc_ecgt signal is used to increment RPTC_CNTR. When cleared, system clock is used instead."]
    #[inline(always)]
    #[must_use]
    pub fn eclk(&mut self) -> EclkW<CtrlSpec> {
        EclkW::new(self, 1)
    }
    #[doc = "Bit 2 - When set, ptc_ecgt increments on negative edge and gates on low period. When cleared, ptc_ecgt increments on positive edge and gates on high period. This bit has effect only on `gating` function of ptc_ecgt when RPTC_CTRL\\[ECLK\\]
bit is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn nec(&mut self) -> NecW<CtrlSpec> {
        NecW::new(self, 2)
    }
    #[doc = "Bit 3 - Inverted value of this bit is reflected on the ptc_oen signal. It is used to enable PWM output driver."]
    #[inline(always)]
    #[must_use]
    pub fn oe(&mut self) -> OeW<CtrlSpec> {
        OeW::new(self, 3)
    }
    #[doc = "Bit 4 - When set, RPTC_CNTR is not incremented anymore after it reaches value equal to the RPTC_LRC value. When cleared, RPTC_CNTR is restarted after it reaches value in the RPTC_LCR register."]
    #[inline(always)]
    #[must_use]
    pub fn single(&mut self) -> SingleW<CtrlSpec> {
        SingleW::new(self, 4)
    }
    #[doc = "Bit 5 - When set, PTC asserts an interrupt when RPTC_CNTR value is equal to the value of RPTC_LRC or RPTC_HRC. When cleared, interrupts are masked."]
    #[inline(always)]
    #[must_use]
    pub fn inte(&mut self) -> InteW<CtrlSpec> {
        InteW::new(self, 5)
    }
    #[doc = "Bit 6 - When read, this bit represents pending interrupt. When it is set, an interrupt is pending. When this bit is written with `1`, interrupt request is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> IntW<CtrlSpec> {
        IntW::new(self, 6)
    }
    #[doc = "Bit 7 - When set, RPTC_CNTR is under reset. When cleared, normal operation of the counter is allowed."]
    #[inline(always)]
    #[must_use]
    pub fn cntrrst(&mut self) -> CntrrstW<CtrlSpec> {
        CntrrstW::new(self, 7)
    }
    #[doc = "Bit 8 - When set, ptc_capt signal can be used to capture RPTC_CNTR into RPTC_LRC or RPTC_HRC registers. Into which reference/capture register capture occurs depends on edge of the ptc_capt signal. When cleared, capture function is masked."]
    #[inline(always)]
    #[must_use]
    pub fn capte(&mut self) -> CapteW<CtrlSpec> {
        CapteW::new(self, 8)
    }
}
#[doc = "Opencores PTC PWM v1 RPTC_CTRL register control operation of PTC core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ctrl to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u16 = 0;
}
