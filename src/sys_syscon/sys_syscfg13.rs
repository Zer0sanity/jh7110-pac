#[doc = "Register `sys_syscfg13` reader"]
pub type R = crate::R<SysSyscfg13Spec>;
#[doc = "Register `sys_syscfg13` writer"]
pub type W = crate::W<SysSyscfg13Spec>;
#[doc = "Field `pll2_prediv` reader - "]
pub type Pll2PredivR = crate::FieldReader;
#[doc = "Field `pll2_prediv` writer - "]
pub type Pll2PredivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `pll2_testen` reader - "]
pub type Pll2TestenR = crate::BitReader;
#[doc = "Field `pll2_testen` writer - "]
pub type Pll2TestenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pll2_testsel` reader - "]
pub type Pll2TestselR = crate::FieldReader;
#[doc = "Field `pll2_testsel` writer - "]
pub type Pll2TestselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pll_test_mode` reader - PLL test mode, only used for PLL BIST through jtag2apb"]
pub type PllTestModeR = crate::BitReader;
#[doc = "Field `pll_test_mode` writer - PLL test mode, only used for PLL BIST through jtag2apb"]
pub type PllTestModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `audio_i2sdin_sel` reader - "]
pub type AudioI2sdinSelR = crate::FieldReader;
#[doc = "Field `audio_i2sdin_sel` writer - "]
pub type AudioI2sdinSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `noc_bus_clock_gating_off` reader - "]
pub type NocBusClockGatingOffR = crate::BitReader;
#[doc = "Field `noc_bus_clock_gating_off` writer - "]
pub type NocBusClockGatingOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_evemon_start0` reader - "]
pub type NocBusOicEvemonStart0R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start0` writer - "]
pub type NocBusOicEvemonStart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_evemon_trigger0` reader - "]
pub type NocBusOicEvemonTrigger0R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start1` reader - "]
pub type NocBusOicEvemonStart1R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start1` writer - "]
pub type NocBusOicEvemonStart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_evemon_trigger1` reader - "]
pub type NocBusOicEvemonTrigger1R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start2` reader - "]
pub type NocBusOicEvemonStart2R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start2` writer - "]
pub type NocBusOicEvemonStart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_evemon_trigger2` reader - "]
pub type NocBusOicEvemonTrigger2R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start3` reader - "]
pub type NocBusOicEvemonStart3R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start3` writer - "]
pub type NocBusOicEvemonStart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_evemon_trigger3` reader - "]
pub type NocBusOicEvemonTrigger3R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start4` reader - "]
pub type NocBusOicEvemonStart4R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start4` writer - "]
pub type NocBusOicEvemonStart4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_evemon_trigger4` reader - "]
pub type NocBusOicEvemonTrigger4R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start5` reader - "]
pub type NocBusOicEvemonStart5R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start5` writer - "]
pub type NocBusOicEvemonStart5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_evemon_trigger5` reader - "]
pub type NocBusOicEvemonTrigger5R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start6` reader - "]
pub type NocBusOicEvemonStart6R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start6` writer - "]
pub type NocBusOicEvemonStart6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn pll2_prediv(&self) -> Pll2PredivR {
        Pll2PredivR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pll2_testen(&self) -> Pll2TestenR {
        Pll2TestenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn pll2_testsel(&self) -> Pll2TestselR {
        Pll2TestselR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - PLL test mode, only used for PLL BIST through jtag2apb"]
    #[inline(always)]
    pub fn pll_test_mode(&self) -> PllTestModeR {
        PllTestModeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:17"]
    #[inline(always)]
    pub fn audio_i2sdin_sel(&self) -> AudioI2sdinSelR {
        AudioI2sdinSelR::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn noc_bus_clock_gating_off(&self) -> NocBusClockGatingOffR {
        NocBusClockGatingOffR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_start0(&self) -> NocBusOicEvemonStart0R {
        NocBusOicEvemonStart0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_trigger0(&self) -> NocBusOicEvemonTrigger0R {
        NocBusOicEvemonTrigger0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_start1(&self) -> NocBusOicEvemonStart1R {
        NocBusOicEvemonStart1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_trigger1(&self) -> NocBusOicEvemonTrigger1R {
        NocBusOicEvemonTrigger1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_start2(&self) -> NocBusOicEvemonStart2R {
        NocBusOicEvemonStart2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_trigger2(&self) -> NocBusOicEvemonTrigger2R {
        NocBusOicEvemonTrigger2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_start3(&self) -> NocBusOicEvemonStart3R {
        NocBusOicEvemonStart3R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_trigger3(&self) -> NocBusOicEvemonTrigger3R {
        NocBusOicEvemonTrigger3R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_start4(&self) -> NocBusOicEvemonStart4R {
        NocBusOicEvemonStart4R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_trigger4(&self) -> NocBusOicEvemonTrigger4R {
        NocBusOicEvemonTrigger4R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_start5(&self) -> NocBusOicEvemonStart5R {
        NocBusOicEvemonStart5R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_trigger5(&self) -> NocBusOicEvemonTrigger5R {
        NocBusOicEvemonTrigger5R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_start6(&self) -> NocBusOicEvemonStart6R {
        NocBusOicEvemonStart6R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn pll2_prediv(&mut self) -> Pll2PredivW<SysSyscfg13Spec> {
        Pll2PredivW::new(self, 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pll2_testen(&mut self) -> Pll2TestenW<SysSyscfg13Spec> {
        Pll2TestenW::new(self, 6)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    #[must_use]
    pub fn pll2_testsel(&mut self) -> Pll2TestselW<SysSyscfg13Spec> {
        Pll2TestselW::new(self, 7)
    }
    #[doc = "Bit 9 - PLL test mode, only used for PLL BIST through jtag2apb"]
    #[inline(always)]
    #[must_use]
    pub fn pll_test_mode(&mut self) -> PllTestModeW<SysSyscfg13Spec> {
        PllTestModeW::new(self, 9)
    }
    #[doc = "Bits 10:17"]
    #[inline(always)]
    #[must_use]
    pub fn audio_i2sdin_sel(&mut self) -> AudioI2sdinSelW<SysSyscfg13Spec> {
        AudioI2sdinSelW::new(self, 10)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_clock_gating_off(&mut self) -> NocBusClockGatingOffW<SysSyscfg13Spec> {
        NocBusClockGatingOffW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_evemon_start0(&mut self) -> NocBusOicEvemonStart0W<SysSyscfg13Spec> {
        NocBusOicEvemonStart0W::new(self, 19)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_evemon_start1(&mut self) -> NocBusOicEvemonStart1W<SysSyscfg13Spec> {
        NocBusOicEvemonStart1W::new(self, 21)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_evemon_start2(&mut self) -> NocBusOicEvemonStart2W<SysSyscfg13Spec> {
        NocBusOicEvemonStart2W::new(self, 23)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_evemon_start3(&mut self) -> NocBusOicEvemonStart3W<SysSyscfg13Spec> {
        NocBusOicEvemonStart3W::new(self, 25)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_evemon_start4(&mut self) -> NocBusOicEvemonStart4W<SysSyscfg13Spec> {
        NocBusOicEvemonStart4W::new(self, 27)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_evemon_start5(&mut self) -> NocBusOicEvemonStart5W<SysSyscfg13Spec> {
        NocBusOicEvemonStart5W::new(self, 29)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_evemon_start6(&mut self) -> NocBusOicEvemonStart6W<SysSyscfg13Spec> {
        NocBusOicEvemonStart6W::new(self, 31)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg13Spec;
impl crate::RegisterSpec for SysSyscfg13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg13::R`](R) reader structure"]
impl crate::Readable for SysSyscfg13Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg13::W`](W) writer structure"]
impl crate::Writable for SysSyscfg13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg13 to value 0x01"]
impl crate::Resettable for SysSyscfg13Spec {
    const RESET_VALUE: u32 = 0x01;
}
