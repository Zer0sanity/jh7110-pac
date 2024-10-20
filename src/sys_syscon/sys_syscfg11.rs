#[doc = "Register `sys_syscfg11` reader"]
pub type R = crate::R<SysSyscfg11Spec>;
#[doc = "Register `sys_syscfg11` writer"]
pub type W = crate::W<SysSyscfg11Spec>;
#[doc = "Field `pll1_prediv` reader - PLL1 prediv value."]
pub type Pll1PredivR = crate::FieldReader;
#[doc = "Field `pll1_prediv` writer - PLL1 prediv value."]
pub type Pll1PredivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `pll1_testen` reader - PLL1 test enable."]
pub type Pll1TestenR = crate::BitReader;
#[doc = "Field `pll1_testen` writer - PLL1 test enable."]
pub type Pll1TestenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pll1_testsel` reader - PLL1 test selector."]
pub type Pll1TestselR = crate::FieldReader;
#[doc = "Field `pll1_testsel` writer - PLL1 test selector."]
pub type Pll1TestselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pll2_cpi_bias` reader - PLL2 CPI bias."]
pub type Pll2CpiBiasR = crate::FieldReader;
#[doc = "Field `pll2_cpi_bias` writer - PLL2 CPI bias."]
pub type Pll2CpiBiasW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pll2_cpp_bias` reader - PLL2 CPP bias."]
pub type Pll2CppBiasR = crate::FieldReader;
#[doc = "Field `pll2_cpp_bias` writer - PLL2 CPP bias."]
pub type Pll2CppBiasW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "PLL2 DACPD.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pll2Dacpd {
    #[doc = "0: Disable PLL2 DACPD."]
    Off = 0,
    #[doc = "1: Enable PLL2 DACPD."]
    On = 1,
}
impl From<Pll2Dacpd> for bool {
    #[inline(always)]
    fn from(variant: Pll2Dacpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll2_dacpd` reader - PLL2 DACPD."]
pub type Pll2DacpdR = crate::BitReader<Pll2Dacpd>;
impl Pll2DacpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pll2Dacpd {
        match self.bits {
            false => Pll2Dacpd::Off,
            true => Pll2Dacpd::On,
        }
    }
    #[doc = "Disable PLL2 DACPD."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pll2Dacpd::Off
    }
    #[doc = "Enable PLL2 DACPD."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pll2Dacpd::On
    }
}
#[doc = "Field `pll2_dacpd` writer - PLL2 DACPD."]
pub type Pll2DacpdW<'a, REG> = crate::BitWriter<'a, REG, Pll2Dacpd>;
impl<'a, REG> Pll2DacpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable PLL2 DACPD."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pll2Dacpd::Off)
    }
    #[doc = "Enable PLL2 DACPD."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pll2Dacpd::On)
    }
}
#[doc = "PLL2 DSMPD.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pll2Dsmpd {
    #[doc = "0: Disable PLL2 DSMPD."]
    Off = 0,
    #[doc = "1: Enable PLL2 DSMPD."]
    On = 1,
}
impl From<Pll2Dsmpd> for bool {
    #[inline(always)]
    fn from(variant: Pll2Dsmpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll2_dsmpd` reader - PLL2 DSMPD."]
pub type Pll2DsmpdR = crate::BitReader<Pll2Dsmpd>;
impl Pll2DsmpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pll2Dsmpd {
        match self.bits {
            false => Pll2Dsmpd::Off,
            true => Pll2Dsmpd::On,
        }
    }
    #[doc = "Disable PLL2 DSMPD."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pll2Dsmpd::Off
    }
    #[doc = "Enable PLL2 DSMPD."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pll2Dsmpd::On
    }
}
#[doc = "Field `pll2_dsmpd` writer - PLL2 DSMPD."]
pub type Pll2DsmpdW<'a, REG> = crate::BitWriter<'a, REG, Pll2Dsmpd>;
impl<'a, REG> Pll2DsmpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable PLL2 DSMPD."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pll2Dsmpd::Off)
    }
    #[doc = "Enable PLL2 DSMPD."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pll2Dsmpd::On)
    }
}
#[doc = "Field `pll2_fbdiv` reader - PLL2 fbdiv value."]
pub type Pll2FbdivR = crate::FieldReader<u16>;
#[doc = "Field `pll2_fbdiv` writer - PLL2 fbdiv value."]
pub type Pll2FbdivW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:5 - PLL1 prediv value."]
    #[inline(always)]
    pub fn pll1_prediv(&self) -> Pll1PredivR {
        Pll1PredivR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - PLL1 test enable."]
    #[inline(always)]
    pub fn pll1_testen(&self) -> Pll1TestenR {
        Pll1TestenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - PLL1 test selector."]
    #[inline(always)]
    pub fn pll1_testsel(&self) -> Pll1TestselR {
        Pll1TestselR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:11 - PLL2 CPI bias."]
    #[inline(always)]
    pub fn pll2_cpi_bias(&self) -> Pll2CpiBiasR {
        Pll2CpiBiasR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - PLL2 CPP bias."]
    #[inline(always)]
    pub fn pll2_cpp_bias(&self) -> Pll2CppBiasR {
        Pll2CppBiasR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - PLL2 DACPD."]
    #[inline(always)]
    pub fn pll2_dacpd(&self) -> Pll2DacpdR {
        Pll2DacpdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PLL2 DSMPD."]
    #[inline(always)]
    pub fn pll2_dsmpd(&self) -> Pll2DsmpdR {
        Pll2DsmpdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:28 - PLL2 fbdiv value."]
    #[inline(always)]
    pub fn pll2_fbdiv(&self) -> Pll2FbdivR {
        Pll2FbdivR::new(((self.bits >> 17) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - PLL1 prediv value."]
    #[inline(always)]
    #[must_use]
    pub fn pll1_prediv(&mut self) -> Pll1PredivW<SysSyscfg11Spec> {
        Pll1PredivW::new(self, 0)
    }
    #[doc = "Bit 6 - PLL1 test enable."]
    #[inline(always)]
    #[must_use]
    pub fn pll1_testen(&mut self) -> Pll1TestenW<SysSyscfg11Spec> {
        Pll1TestenW::new(self, 6)
    }
    #[doc = "Bits 7:8 - PLL1 test selector."]
    #[inline(always)]
    #[must_use]
    pub fn pll1_testsel(&mut self) -> Pll1TestselW<SysSyscfg11Spec> {
        Pll1TestselW::new(self, 7)
    }
    #[doc = "Bits 9:11 - PLL2 CPI bias."]
    #[inline(always)]
    #[must_use]
    pub fn pll2_cpi_bias(&mut self) -> Pll2CpiBiasW<SysSyscfg11Spec> {
        Pll2CpiBiasW::new(self, 9)
    }
    #[doc = "Bits 12:14 - PLL2 CPP bias."]
    #[inline(always)]
    #[must_use]
    pub fn pll2_cpp_bias(&mut self) -> Pll2CppBiasW<SysSyscfg11Spec> {
        Pll2CppBiasW::new(self, 12)
    }
    #[doc = "Bit 15 - PLL2 DACPD."]
    #[inline(always)]
    #[must_use]
    pub fn pll2_dacpd(&mut self) -> Pll2DacpdW<SysSyscfg11Spec> {
        Pll2DacpdW::new(self, 15)
    }
    #[doc = "Bit 16 - PLL2 DSMPD."]
    #[inline(always)]
    #[must_use]
    pub fn pll2_dsmpd(&mut self) -> Pll2DsmpdW<SysSyscfg11Spec> {
        Pll2DsmpdW::new(self, 16)
    }
    #[doc = "Bits 17:28 - PLL2 fbdiv value."]
    #[inline(always)]
    #[must_use]
    pub fn pll2_fbdiv(&mut self) -> Pll2FbdivW<SysSyscfg11Spec> {
        Pll2FbdivW::new(self, 17)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg11Spec;
impl crate::RegisterSpec for SysSyscfg11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg11::R`](R) reader structure"]
impl crate::Readable for SysSyscfg11Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg11::W`](W) writer structure"]
impl crate::Writable for SysSyscfg11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg11 to value 0x0066_2601"]
impl crate::Resettable for SysSyscfg11Spec {
    const RESET_VALUE: u32 = 0x0066_2601;
}
