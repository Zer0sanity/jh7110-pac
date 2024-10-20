#[doc = "Register `sys_syscfg10` reader"]
pub type R = crate::R<SysSyscfg10Spec>;
#[doc = "Register `sys_syscfg10` writer"]
pub type W = crate::W<SysSyscfg10Spec>;
#[doc = "Field `pll1_frac` reader - PLL1 frac value."]
pub type Pll1FracR = crate::FieldReader<u32>;
#[doc = "Field `pll1_frac` writer - PLL1 frac value."]
pub type Pll1FracW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `pll1_gvco_bias` reader - PLL1 GVCO bias."]
pub type Pll1GvcoBiasR = crate::FieldReader;
#[doc = "Field `pll1_gvco_bias` writer - PLL1 GVCO bias."]
pub type Pll1GvcoBiasW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pll1_lock` reader - PLL1 lock."]
pub type Pll1LockR = crate::BitReader;
#[doc = "PLL1 PD enable setting - driving the register low turns PD `on`.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pll1Pd {
    #[doc = "0: Enable the PLL0 PD"]
    On = 0,
    #[doc = "1: Disable the PLL0 PD"]
    Off = 1,
}
impl From<Pll1Pd> for bool {
    #[inline(always)]
    fn from(variant: Pll1Pd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll1_pd` reader - PLL1 PD enable setting - driving the register low turns PD `on`."]
pub type Pll1PdR = crate::BitReader<Pll1Pd>;
impl Pll1PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pll1Pd {
        match self.bits {
            false => Pll1Pd::On,
            true => Pll1Pd::Off,
        }
    }
    #[doc = "Enable the PLL0 PD"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pll1Pd::On
    }
    #[doc = "Disable the PLL0 PD"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pll1Pd::Off
    }
}
#[doc = "Field `pll1_pd` writer - PLL1 PD enable setting - driving the register low turns PD `on`."]
pub type Pll1PdW<'a, REG> = crate::BitWriter<'a, REG, Pll1Pd>;
impl<'a, REG> Pll1PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the PLL0 PD"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pll1Pd::On)
    }
    #[doc = "Disable the PLL0 PD"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pll1Pd::Off)
    }
}
#[doc = "Field `pll1_postdiv1` reader - PLL1 postdiv1 value."]
pub type Pll1Postdiv1R = crate::FieldReader;
#[doc = "Field `pll1_postdiv1` writer - PLL1 postdiv1 value."]
pub type Pll1Postdiv1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pll1_postdiv2` reader - PLL1 postdiv2 value."]
pub type Pll1Postdiv2R = crate::FieldReader;
#[doc = "Field `pll1_postdiv2` writer - PLL1 postdiv2 value."]
pub type Pll1Postdiv2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:23 - PLL1 frac value."]
    #[inline(always)]
    pub fn pll1_frac(&self) -> Pll1FracR {
        Pll1FracR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:25 - PLL1 GVCO bias."]
    #[inline(always)]
    pub fn pll1_gvco_bias(&self) -> Pll1GvcoBiasR {
        Pll1GvcoBiasR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - PLL1 lock."]
    #[inline(always)]
    pub fn pll1_lock(&self) -> Pll1LockR {
        Pll1LockR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PLL1 PD enable setting - driving the register low turns PD `on`."]
    #[inline(always)]
    pub fn pll1_pd(&self) -> Pll1PdR {
        Pll1PdR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - PLL1 postdiv1 value."]
    #[inline(always)]
    pub fn pll1_postdiv1(&self) -> Pll1Postdiv1R {
        Pll1Postdiv1R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PLL1 postdiv2 value."]
    #[inline(always)]
    pub fn pll1_postdiv2(&self) -> Pll1Postdiv2R {
        Pll1Postdiv2R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - PLL1 frac value."]
    #[inline(always)]
    #[must_use]
    pub fn pll1_frac(&mut self) -> Pll1FracW<SysSyscfg10Spec> {
        Pll1FracW::new(self, 0)
    }
    #[doc = "Bits 24:25 - PLL1 GVCO bias."]
    #[inline(always)]
    #[must_use]
    pub fn pll1_gvco_bias(&mut self) -> Pll1GvcoBiasW<SysSyscfg10Spec> {
        Pll1GvcoBiasW::new(self, 24)
    }
    #[doc = "Bit 27 - PLL1 PD enable setting - driving the register low turns PD `on`."]
    #[inline(always)]
    #[must_use]
    pub fn pll1_pd(&mut self) -> Pll1PdW<SysSyscfg10Spec> {
        Pll1PdW::new(self, 27)
    }
    #[doc = "Bits 28:29 - PLL1 postdiv1 value."]
    #[inline(always)]
    #[must_use]
    pub fn pll1_postdiv1(&mut self) -> Pll1Postdiv1W<SysSyscfg10Spec> {
        Pll1Postdiv1W::new(self, 28)
    }
    #[doc = "Bits 30:31 - PLL1 postdiv2 value."]
    #[inline(always)]
    #[must_use]
    pub fn pll1_postdiv2(&mut self) -> Pll1Postdiv2W<SysSyscfg10Spec> {
        Pll1Postdiv2W::new(self, 30)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg10Spec;
impl crate::RegisterSpec for SysSyscfg10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg10::R`](R) reader structure"]
impl crate::Readable for SysSyscfg10Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg10::W`](W) writer structure"]
impl crate::Writable for SysSyscfg10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg10 to value 0x51e0_0000"]
impl crate::Resettable for SysSyscfg10Spec {
    const RESET_VALUE: u32 = 0x51e0_0000;
}
