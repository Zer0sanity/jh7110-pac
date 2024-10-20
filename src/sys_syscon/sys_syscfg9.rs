#[doc = "Register `sys_syscfg9` reader"]
pub type R = crate::R<SysSyscfg9Spec>;
#[doc = "Register `sys_syscfg9` writer"]
pub type W = crate::W<SysSyscfg9Spec>;
#[doc = "Field `pll0_prediv` reader - "]
pub type Pll0PredivR = crate::FieldReader;
#[doc = "Field `pll0_prediv` writer - "]
pub type Pll0PredivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `pll0_testen` reader - "]
pub type Pll0TestenR = crate::BitReader;
#[doc = "Field `pll0_testen` writer - "]
pub type Pll0TestenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pll0_testsel` reader - "]
pub type Pll0TestselR = crate::FieldReader;
#[doc = "Field `pll0_testsel` writer - "]
pub type Pll0TestselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pll1_cpi_bias` reader - "]
pub type Pll1CpiBiasR = crate::FieldReader;
#[doc = "Field `pll1_cpi_bias` writer - "]
pub type Pll1CpiBiasW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pll1_cpp_bias` reader - "]
pub type Pll1CppBiasR = crate::FieldReader;
#[doc = "Field `pll1_cpp_bias` writer - "]
pub type Pll1CppBiasW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "PLL1 DACPD.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pll1Dacpd {
    #[doc = "0: Disable PLL1 DACPD."]
    Off = 0,
    #[doc = "1: Enable PLL1 DACPD."]
    On = 1,
}
impl From<Pll1Dacpd> for bool {
    #[inline(always)]
    fn from(variant: Pll1Dacpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll1_dacpd` reader - PLL1 DACPD."]
pub type Pll1DacpdR = crate::BitReader<Pll1Dacpd>;
impl Pll1DacpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pll1Dacpd {
        match self.bits {
            false => Pll1Dacpd::Off,
            true => Pll1Dacpd::On,
        }
    }
    #[doc = "Disable PLL1 DACPD."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pll1Dacpd::Off
    }
    #[doc = "Enable PLL1 DACPD."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pll1Dacpd::On
    }
}
#[doc = "Field `pll1_dacpd` writer - PLL1 DACPD."]
pub type Pll1DacpdW<'a, REG> = crate::BitWriter<'a, REG, Pll1Dacpd>;
impl<'a, REG> Pll1DacpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable PLL1 DACPD."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pll1Dacpd::Off)
    }
    #[doc = "Enable PLL1 DACPD."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pll1Dacpd::On)
    }
}
#[doc = "PLL1 DSMPD.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pll1Dsmpd {
    #[doc = "0: Disable PLL1 DSMPD."]
    Off = 0,
    #[doc = "1: Enable PLL1 DSMPD."]
    On = 1,
}
impl From<Pll1Dsmpd> for bool {
    #[inline(always)]
    fn from(variant: Pll1Dsmpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll1_dsmpd` reader - PLL1 DSMPD."]
pub type Pll1DsmpdR = crate::BitReader<Pll1Dsmpd>;
impl Pll1DsmpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pll1Dsmpd {
        match self.bits {
            false => Pll1Dsmpd::Off,
            true => Pll1Dsmpd::On,
        }
    }
    #[doc = "Disable PLL1 DSMPD."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pll1Dsmpd::Off
    }
    #[doc = "Enable PLL1 DSMPD."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pll1Dsmpd::On
    }
}
#[doc = "Field `pll1_dsmpd` writer - PLL1 DSMPD."]
pub type Pll1DsmpdW<'a, REG> = crate::BitWriter<'a, REG, Pll1Dsmpd>;
impl<'a, REG> Pll1DsmpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable PLL1 DSMPD."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pll1Dsmpd::Off)
    }
    #[doc = "Enable PLL1 DSMPD."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pll1Dsmpd::On)
    }
}
#[doc = "Field `pll1_fbdiv` reader - "]
pub type Pll1FbdivR = crate::FieldReader<u16>;
#[doc = "Field `pll1_fbdiv` writer - "]
pub type Pll1FbdivW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn pll0_prediv(&self) -> Pll0PredivR {
        Pll0PredivR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pll0_testen(&self) -> Pll0TestenR {
        Pll0TestenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn pll0_testsel(&self) -> Pll0TestselR {
        Pll0TestselR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn pll1_cpi_bias(&self) -> Pll1CpiBiasR {
        Pll1CpiBiasR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn pll1_cpp_bias(&self) -> Pll1CppBiasR {
        Pll1CppBiasR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - PLL1 DACPD."]
    #[inline(always)]
    pub fn pll1_dacpd(&self) -> Pll1DacpdR {
        Pll1DacpdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PLL1 DSMPD."]
    #[inline(always)]
    pub fn pll1_dsmpd(&self) -> Pll1DsmpdR {
        Pll1DsmpdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:28"]
    #[inline(always)]
    pub fn pll1_fbdiv(&self) -> Pll1FbdivR {
        Pll1FbdivR::new(((self.bits >> 17) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn pll0_prediv(&mut self) -> Pll0PredivW<SysSyscfg9Spec> {
        Pll0PredivW::new(self, 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pll0_testen(&mut self) -> Pll0TestenW<SysSyscfg9Spec> {
        Pll0TestenW::new(self, 6)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    #[must_use]
    pub fn pll0_testsel(&mut self) -> Pll0TestselW<SysSyscfg9Spec> {
        Pll0TestselW::new(self, 7)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    #[must_use]
    pub fn pll1_cpi_bias(&mut self) -> Pll1CpiBiasW<SysSyscfg9Spec> {
        Pll1CpiBiasW::new(self, 9)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn pll1_cpp_bias(&mut self) -> Pll1CppBiasW<SysSyscfg9Spec> {
        Pll1CppBiasW::new(self, 12)
    }
    #[doc = "Bit 15 - PLL1 DACPD."]
    #[inline(always)]
    #[must_use]
    pub fn pll1_dacpd(&mut self) -> Pll1DacpdW<SysSyscfg9Spec> {
        Pll1DacpdW::new(self, 15)
    }
    #[doc = "Bit 16 - PLL1 DSMPD."]
    #[inline(always)]
    #[must_use]
    pub fn pll1_dsmpd(&mut self) -> Pll1DsmpdW<SysSyscfg9Spec> {
        Pll1DsmpdW::new(self, 16)
    }
    #[doc = "Bits 17:28"]
    #[inline(always)]
    #[must_use]
    pub fn pll1_fbdiv(&mut self) -> Pll1FbdivW<SysSyscfg9Spec> {
        Pll1FbdivW::new(self, 17)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg9Spec;
impl crate::RegisterSpec for SysSyscfg9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg9::R`](R) reader structure"]
impl crate::Readable for SysSyscfg9Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg9::W`](W) writer structure"]
impl crate::Writable for SysSyscfg9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg9 to value 0x00b0_2601"]
impl crate::Resettable for SysSyscfg9Spec {
    const RESET_VALUE: u32 = 0x00b0_2601;
}
