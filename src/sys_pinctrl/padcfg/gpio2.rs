#[doc = "Register `gpio2` reader"]
pub type R = crate::R<Gpio2Spec>;
#[doc = "Register `gpio2` writer"]
pub type W = crate::W<Gpio2Spec>;
#[doc = "Field `ie` reader - Input Enable (IE) Controller - 1: Enable the receiver, 0: Disable the receiver"]
pub type IeR = crate::BitReader;
#[doc = "Field `ie` writer - Input Enable (IE) Controller - 1: Enable the receiver, 0: Disable the receiver"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ds` reader - Output Drive Strength (DS) - 00: The rated drive strength is 2 mA, 01: The rated drive strength is 4 mA, 10: The rated drive strength is 8 mA, 11: The rated drive strength is 12 mA"]
pub type DsR = crate::FieldReader;
#[doc = "Field `ds` writer - Output Drive Strength (DS) - 00: The rated drive strength is 2 mA, 01: The rated drive strength is 4 mA, 10: The rated drive strength is 8 mA, 11: The rated drive strength is 12 mA"]
pub type DsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pu` reader - Pull-Up (PU) settings - 1: Yes, 0: No"]
pub type PuR = crate::BitReader;
#[doc = "Field `pu` writer - Pull-Up (PU) settings - 1: Yes, 0: No"]
pub type PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pd` reader - Pull-Down (PD) settings - 1: Yes, 0: No"]
pub type PdR = crate::BitReader;
#[doc = "Field `pd` writer - Pull-Down (PD) settings - 1: Yes, 0: No"]
pub type PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slew` reader - Slew Rate Control - 0: Slow (Half frequency), 1: Fast"]
pub type SlewR = crate::BitReader;
#[doc = "Field `slew` writer - Slew Rate Control - 0: Slow (Half frequency), 1: Fast"]
pub type SlewW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `smt` reader - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger ebabled"]
pub type SmtR = crate::BitReader;
#[doc = "Field `smt` writer - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger ebabled"]
pub type SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pos` reader - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
pub type PosR = crate::BitReader;
#[doc = "Field `pos` writer - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
pub type PosW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Input Enable (IE) Controller - 1: Enable the receiver, 0: Disable the receiver"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Output Drive Strength (DS) - 00: The rated drive strength is 2 mA, 01: The rated drive strength is 4 mA, 10: The rated drive strength is 8 mA, 11: The rated drive strength is 12 mA"]
    #[inline(always)]
    pub fn ds(&self) -> DsR {
        DsR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Pull-Up (PU) settings - 1: Yes, 0: No"]
    #[inline(always)]
    pub fn pu(&self) -> PuR {
        PuR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pull-Down (PD) settings - 1: Yes, 0: No"]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slew Rate Control - 0: Slow (Half frequency), 1: Fast"]
    #[inline(always)]
    pub fn slew(&self) -> SlewR {
        SlewR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger ebabled"]
    #[inline(always)]
    pub fn smt(&self) -> SmtR {
        SmtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
    #[inline(always)]
    pub fn pos(&self) -> PosR {
        PosR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input Enable (IE) Controller - 1: Enable the receiver, 0: Disable the receiver"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IeW<Gpio2Spec> {
        IeW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Output Drive Strength (DS) - 00: The rated drive strength is 2 mA, 01: The rated drive strength is 4 mA, 10: The rated drive strength is 8 mA, 11: The rated drive strength is 12 mA"]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DsW<Gpio2Spec> {
        DsW::new(self, 1)
    }
    #[doc = "Bit 3 - Pull-Up (PU) settings - 1: Yes, 0: No"]
    #[inline(always)]
    #[must_use]
    pub fn pu(&mut self) -> PuW<Gpio2Spec> {
        PuW::new(self, 3)
    }
    #[doc = "Bit 4 - Pull-Down (PD) settings - 1: Yes, 0: No"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PdW<Gpio2Spec> {
        PdW::new(self, 4)
    }
    #[doc = "Bit 5 - Slew Rate Control - 0: Slow (Half frequency), 1: Fast"]
    #[inline(always)]
    #[must_use]
    pub fn slew(&mut self) -> SlewW<Gpio2Spec> {
        SlewW::new(self, 5)
    }
    #[doc = "Bit 6 - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger ebabled"]
    #[inline(always)]
    #[must_use]
    pub fn smt(&mut self) -> SmtW<Gpio2Spec> {
        SmtW::new(self, 6)
    }
    #[doc = "Bit 7 - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
    #[inline(always)]
    #[must_use]
    pub fn pos(&mut self) -> PosW<Gpio2Spec> {
        PosW::new(self, 7)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 296: GPIO2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio2Spec;
impl crate::RegisterSpec for Gpio2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio2::R`](R) reader structure"]
impl crate::Readable for Gpio2Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio2::W`](W) writer structure"]
impl crate::Writable for Gpio2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpio2 to value 0x01"]
impl crate::Resettable for Gpio2Spec {
    const RESET_VALUE: u32 = 0x01;
}
