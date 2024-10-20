#[doc = "Register `sys_syscfg8` reader"]
pub type R = crate::R<SysSyscfg8Spec>;
#[doc = "Register `sys_syscfg8` writer"]
pub type W = crate::W<SysSyscfg8Spec>;
#[doc = "Field `pll0_frac` reader - PLL0 frac value."]
pub type Pll0FracR = crate::FieldReader<u32>;
#[doc = "Field `pll0_frac` writer - PLL0 frac value."]
pub type Pll0FracW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `pll0_gvco_bias` reader - PLL0 GVCO bias."]
pub type Pll0GvcoBiasR = crate::FieldReader;
#[doc = "Field `pll0_gvco_bias` writer - PLL0 GVCO bias."]
pub type Pll0GvcoBiasW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pll0_lock` reader - PLL0 lock."]
pub type Pll0LockR = crate::BitReader;
#[doc = "PLL0 PD enable setting - driving the register low turns PD `on`.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pll0Pd {
    #[doc = "0: Enable the PLL0 PD."]
    On = 0,
    #[doc = "1: Disable the PLL0 PD."]
    Off = 1,
}
impl From<Pll0Pd> for bool {
    #[inline(always)]
    fn from(variant: Pll0Pd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll0_pd` reader - PLL0 PD enable setting - driving the register low turns PD `on`."]
pub type Pll0PdR = crate::BitReader<Pll0Pd>;
impl Pll0PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pll0Pd {
        match self.bits {
            false => Pll0Pd::On,
            true => Pll0Pd::Off,
        }
    }
    #[doc = "Enable the PLL0 PD."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pll0Pd::On
    }
    #[doc = "Disable the PLL0 PD."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pll0Pd::Off
    }
}
#[doc = "Field `pll0_pd` writer - PLL0 PD enable setting - driving the register low turns PD `on`."]
pub type Pll0PdW<'a, REG> = crate::BitWriter<'a, REG, Pll0Pd>;
impl<'a, REG> Pll0PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the PLL0 PD."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pll0Pd::On)
    }
    #[doc = "Disable the PLL0 PD."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pll0Pd::Off)
    }
}
#[doc = "Field `pll0_postdiv1` reader - PLL0 postdiv1 value."]
pub type Pll0Postdiv1R = crate::FieldReader;
#[doc = "Field `pll0_postdiv1` writer - PLL0 postdiv1 value."]
pub type Pll0Postdiv1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pll0_postdiv2` reader - PLL0 postdiv2 value."]
pub type Pll0Postdiv2R = crate::FieldReader;
#[doc = "Field `pll0_postdiv2` writer - PLL0 postdiv2 value."]
pub type Pll0Postdiv2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:23 - PLL0 frac value."]
    #[inline(always)]
    pub fn pll0_frac(&self) -> Pll0FracR {
        Pll0FracR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:25 - PLL0 GVCO bias."]
    #[inline(always)]
    pub fn pll0_gvco_bias(&self) -> Pll0GvcoBiasR {
        Pll0GvcoBiasR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - PLL0 lock."]
    #[inline(always)]
    pub fn pll0_lock(&self) -> Pll0LockR {
        Pll0LockR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PLL0 PD enable setting - driving the register low turns PD `on`."]
    #[inline(always)]
    pub fn pll0_pd(&self) -> Pll0PdR {
        Pll0PdR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - PLL0 postdiv1 value."]
    #[inline(always)]
    pub fn pll0_postdiv1(&self) -> Pll0Postdiv1R {
        Pll0Postdiv1R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PLL0 postdiv2 value."]
    #[inline(always)]
    pub fn pll0_postdiv2(&self) -> Pll0Postdiv2R {
        Pll0Postdiv2R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - PLL0 frac value."]
    #[inline(always)]
    #[must_use]
    pub fn pll0_frac(&mut self) -> Pll0FracW<SysSyscfg8Spec> {
        Pll0FracW::new(self, 0)
    }
    #[doc = "Bits 24:25 - PLL0 GVCO bias."]
    #[inline(always)]
    #[must_use]
    pub fn pll0_gvco_bias(&mut self) -> Pll0GvcoBiasW<SysSyscfg8Spec> {
        Pll0GvcoBiasW::new(self, 24)
    }
    #[doc = "Bit 27 - PLL0 PD enable setting - driving the register low turns PD `on`."]
    #[inline(always)]
    #[must_use]
    pub fn pll0_pd(&mut self) -> Pll0PdW<SysSyscfg8Spec> {
        Pll0PdW::new(self, 27)
    }
    #[doc = "Bits 28:29 - PLL0 postdiv1 value."]
    #[inline(always)]
    #[must_use]
    pub fn pll0_postdiv1(&mut self) -> Pll0Postdiv1W<SysSyscfg8Spec> {
        Pll0Postdiv1W::new(self, 28)
    }
    #[doc = "Bits 30:31 - PLL0 postdiv2 value."]
    #[inline(always)]
    #[must_use]
    pub fn pll0_postdiv2(&mut self) -> Pll0Postdiv2W<SysSyscfg8Spec> {
        Pll0Postdiv2W::new(self, 30)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg8Spec;
impl crate::RegisterSpec for SysSyscfg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg8::R`](R) reader structure"]
impl crate::Readable for SysSyscfg8Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg8::W`](W) writer structure"]
impl crate::Writable for SysSyscfg8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg8 to value 0x5155_5555"]
impl crate::Resettable for SysSyscfg8Spec {
    const RESET_VALUE: u32 = 0x5155_5555;
}
