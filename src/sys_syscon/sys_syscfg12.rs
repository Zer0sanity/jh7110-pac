#[doc = "Register `sys_syscfg12` reader"]
pub type R = crate::R<SysSyscfg12Spec>;
#[doc = "Register `sys_syscfg12` writer"]
pub type W = crate::W<SysSyscfg12Spec>;
#[doc = "Field `pll2_frac` reader - PLL2 frac value."]
pub type Pll2FracR = crate::FieldReader<u32>;
#[doc = "Field `pll2_frac` writer - PLL2 frac value."]
pub type Pll2FracW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `pll2_gvco_bias` reader - PLL2 GVCO bias."]
pub type Pll2GvcoBiasR = crate::FieldReader;
#[doc = "Field `pll2_gvco_bias` writer - PLL2 GVCO bias."]
pub type Pll2GvcoBiasW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pll2_lock` reader - PLL2 lock."]
pub type Pll2LockR = crate::BitReader;
#[doc = "PLL2 PD enable setting - driving the register low turns PD `on`.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pll2Pd {
    #[doc = "0: Enable the PLL2 PD."]
    On = 0,
    #[doc = "1: Disable the PLL2 PD."]
    Off = 1,
}
impl From<Pll2Pd> for bool {
    #[inline(always)]
    fn from(variant: Pll2Pd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll2_pd` reader - PLL2 PD enable setting - driving the register low turns PD `on`."]
pub type Pll2PdR = crate::BitReader<Pll2Pd>;
impl Pll2PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pll2Pd {
        match self.bits {
            false => Pll2Pd::On,
            true => Pll2Pd::Off,
        }
    }
    #[doc = "Enable the PLL2 PD."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pll2Pd::On
    }
    #[doc = "Disable the PLL2 PD."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pll2Pd::Off
    }
}
#[doc = "Field `pll2_pd` writer - PLL2 PD enable setting - driving the register low turns PD `on`."]
pub type Pll2PdW<'a, REG> = crate::BitWriter<'a, REG, Pll2Pd>;
impl<'a, REG> Pll2PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the PLL2 PD."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pll2Pd::On)
    }
    #[doc = "Disable the PLL2 PD."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pll2Pd::Off)
    }
}
#[doc = "Field `pll2_postdiv1` reader - PLL2 postdiv1 value."]
pub type Pll2Postdiv1R = crate::FieldReader;
#[doc = "Field `pll2_postdiv1` writer - PLL2 postdiv1 value."]
pub type Pll2Postdiv1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pll2_postdiv2` reader - PLL2 postdiv2 value."]
pub type Pll2Postdiv2R = crate::FieldReader;
#[doc = "Field `pll2_postdiv2` writer - PLL2 postdiv2 value."]
pub type Pll2Postdiv2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:23 - PLL2 frac value."]
    #[inline(always)]
    pub fn pll2_frac(&self) -> Pll2FracR {
        Pll2FracR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:25 - PLL2 GVCO bias."]
    #[inline(always)]
    pub fn pll2_gvco_bias(&self) -> Pll2GvcoBiasR {
        Pll2GvcoBiasR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - PLL2 lock."]
    #[inline(always)]
    pub fn pll2_lock(&self) -> Pll2LockR {
        Pll2LockR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PLL2 PD enable setting - driving the register low turns PD `on`."]
    #[inline(always)]
    pub fn pll2_pd(&self) -> Pll2PdR {
        Pll2PdR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - PLL2 postdiv1 value."]
    #[inline(always)]
    pub fn pll2_postdiv1(&self) -> Pll2Postdiv1R {
        Pll2Postdiv1R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PLL2 postdiv2 value."]
    #[inline(always)]
    pub fn pll2_postdiv2(&self) -> Pll2Postdiv2R {
        Pll2Postdiv2R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - PLL2 frac value."]
    #[inline(always)]
    #[must_use]
    pub fn pll2_frac(&mut self) -> Pll2FracW<SysSyscfg12Spec> {
        Pll2FracW::new(self, 0)
    }
    #[doc = "Bits 24:25 - PLL2 GVCO bias."]
    #[inline(always)]
    #[must_use]
    pub fn pll2_gvco_bias(&mut self) -> Pll2GvcoBiasW<SysSyscfg12Spec> {
        Pll2GvcoBiasW::new(self, 24)
    }
    #[doc = "Bit 27 - PLL2 PD enable setting - driving the register low turns PD `on`."]
    #[inline(always)]
    #[must_use]
    pub fn pll2_pd(&mut self) -> Pll2PdW<SysSyscfg12Spec> {
        Pll2PdW::new(self, 27)
    }
    #[doc = "Bits 28:29 - PLL2 postdiv1 value."]
    #[inline(always)]
    #[must_use]
    pub fn pll2_postdiv1(&mut self) -> Pll2Postdiv1W<SysSyscfg12Spec> {
        Pll2Postdiv1W::new(self, 28)
    }
    #[doc = "Bits 30:31 - PLL2 postdiv2 value."]
    #[inline(always)]
    #[must_use]
    pub fn pll2_postdiv2(&mut self) -> Pll2Postdiv2W<SysSyscfg12Spec> {
        Pll2Postdiv2W::new(self, 30)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg12Spec;
impl crate::RegisterSpec for SysSyscfg12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg12::R`](R) reader structure"]
impl crate::Readable for SysSyscfg12Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg12::W`](W) writer structure"]
impl crate::Writable for SysSyscfg12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg12 to value 0x4133_3333"]
impl crate::Resettable for SysSyscfg12Spec {
    const RESET_VALUE: u32 = 0x4133_3333;
}
