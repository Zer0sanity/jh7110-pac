#[doc = "Register `usb_pwr` reader"]
pub type R = crate::R<UsbPwrSpec>;
#[doc = "Register `usb_pwr` writer"]
pub type W = crate::W<UsbPwrSpec>;
#[doc = "Field `pso(_en,_ds)` reader - Power Shutoff capability enable/disable - pso0: enable, pso1: disable"]
pub type PsoR = crate::BitReader;
#[doc = "Field `pso(_en,_ds)` writer - Power Shutoff capability enable/disable - pso0: enable, pso1: disable"]
pub type PsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stb_clk_switch(_en,_done)` reader - Reference clock switch, only enabled if OTG_READY set to `1` - stb_clk_switch0: enable, stb_clk_switch1: done"]
pub type StbClkSwitchR = crate::BitReader;
#[doc = "Field `stb_clk_switch(_en,_done)` writer - Reference clock switch, only enabled if OTG_READY set to `1` - stb_clk_switch0: enable, stb_clk_switch1: done"]
pub type StbClkSwitchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fast_reg_access(_stat,_en)` reader - Fast Register Access - fast_reg_access0: status, fast_reg_access1: enable"]
pub type FastRegAccessR = crate::BitReader;
#[doc = "Field `fast_reg_access(_stat,_en)` writer - Fast Register Access - fast_reg_access0: status, fast_reg_access1: enable"]
pub type FastRegAccessW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Power Shutoff capability enable/disable - pso0: enable, pso1: disable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `pso_en` field"]
    #[inline(always)]
    pub fn pso(&self, n: u8) -> PsoR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        PsoR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Power Shutoff capability enable/disable - pso0: enable, pso1: disable"]
    #[inline(always)]
    pub fn pso_iter(&self) -> impl Iterator<Item = PsoR> + '_ {
        (0..2).map(move |n| PsoR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Power Shutoff capability enable/disable - pso0: enable, pso1: disable"]
    #[inline(always)]
    pub fn pso_en(&self) -> PsoR {
        PsoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Shutoff capability enable/disable - pso0: enable, pso1: disable"]
    #[inline(always)]
    pub fn pso_ds(&self) -> PsoR {
        PsoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Reference clock switch, only enabled if OTG_READY set to `1` - stb_clk_switch0: enable, stb_clk_switch1: done"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `stb_clk_switch_en` field"]
    #[inline(always)]
    pub fn stb_clk_switch(&self, n: u8) -> StbClkSwitchR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        StbClkSwitchR::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Reference clock switch, only enabled if OTG_READY set to `1` - stb_clk_switch0: enable, stb_clk_switch1: done"]
    #[inline(always)]
    pub fn stb_clk_switch_iter(&self) -> impl Iterator<Item = StbClkSwitchR> + '_ {
        (0..2).map(move |n| StbClkSwitchR::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    #[doc = "Bit 8 - Reference clock switch, only enabled if OTG_READY set to `1` - stb_clk_switch0: enable, stb_clk_switch1: done"]
    #[inline(always)]
    pub fn stb_clk_switch_en(&self) -> StbClkSwitchR {
        StbClkSwitchR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reference clock switch, only enabled if OTG_READY set to `1` - stb_clk_switch0: enable, stb_clk_switch1: done"]
    #[inline(always)]
    pub fn stb_clk_switch_done(&self) -> StbClkSwitchR {
        StbClkSwitchR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Fast Register Access - fast_reg_access0: status, fast_reg_access1: enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `fast_reg_access_stat` field"]
    #[inline(always)]
    pub fn fast_reg_access(&self, n: u8) -> FastRegAccessR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        FastRegAccessR::new(((self.bits >> (n + 30)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Fast Register Access - fast_reg_access0: status, fast_reg_access1: enable"]
    #[inline(always)]
    pub fn fast_reg_access_iter(&self) -> impl Iterator<Item = FastRegAccessR> + '_ {
        (0..2).map(move |n| FastRegAccessR::new(((self.bits >> (n + 30)) & 1) != 0))
    }
    #[doc = "Bit 30 - Fast Register Access - fast_reg_access0: status, fast_reg_access1: enable"]
    #[inline(always)]
    pub fn fast_reg_access_stat(&self) -> FastRegAccessR {
        FastRegAccessR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Fast Register Access - fast_reg_access0: status, fast_reg_access1: enable"]
    #[inline(always)]
    pub fn fast_reg_access_en(&self) -> FastRegAccessR {
        FastRegAccessR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Power Shutoff capability enable/disable - pso0: enable, pso1: disable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `pso_en` field"]
    #[inline(always)]
    #[must_use]
    pub fn pso(&mut self, n: u8) -> PsoW<UsbPwrSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        PsoW::new(self, n)
    }
    #[doc = "Bit 0 - Power Shutoff capability enable/disable - pso0: enable, pso1: disable"]
    #[inline(always)]
    #[must_use]
    pub fn pso_en(&mut self) -> PsoW<UsbPwrSpec> {
        PsoW::new(self, 0)
    }
    #[doc = "Bit 1 - Power Shutoff capability enable/disable - pso0: enable, pso1: disable"]
    #[inline(always)]
    #[must_use]
    pub fn pso_ds(&mut self) -> PsoW<UsbPwrSpec> {
        PsoW::new(self, 1)
    }
    #[doc = "Reference clock switch, only enabled if OTG_READY set to `1` - stb_clk_switch0: enable, stb_clk_switch1: done"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `stb_clk_switch_en` field"]
    #[inline(always)]
    #[must_use]
    pub fn stb_clk_switch(&mut self, n: u8) -> StbClkSwitchW<UsbPwrSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        StbClkSwitchW::new(self, n + 8)
    }
    #[doc = "Bit 8 - Reference clock switch, only enabled if OTG_READY set to `1` - stb_clk_switch0: enable, stb_clk_switch1: done"]
    #[inline(always)]
    #[must_use]
    pub fn stb_clk_switch_en(&mut self) -> StbClkSwitchW<UsbPwrSpec> {
        StbClkSwitchW::new(self, 8)
    }
    #[doc = "Bit 9 - Reference clock switch, only enabled if OTG_READY set to `1` - stb_clk_switch0: enable, stb_clk_switch1: done"]
    #[inline(always)]
    #[must_use]
    pub fn stb_clk_switch_done(&mut self) -> StbClkSwitchW<UsbPwrSpec> {
        StbClkSwitchW::new(self, 9)
    }
    #[doc = "Fast Register Access - fast_reg_access0: status, fast_reg_access1: enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `fast_reg_access_stat` field"]
    #[inline(always)]
    #[must_use]
    pub fn fast_reg_access(&mut self, n: u8) -> FastRegAccessW<UsbPwrSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        FastRegAccessW::new(self, n + 30)
    }
    #[doc = "Bit 30 - Fast Register Access - fast_reg_access0: status, fast_reg_access1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn fast_reg_access_stat(&mut self) -> FastRegAccessW<UsbPwrSpec> {
        FastRegAccessW::new(self, 30)
    }
    #[doc = "Bit 31 - Fast Register Access - fast_reg_access0: status, fast_reg_access1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn fast_reg_access_en(&mut self) -> FastRegAccessW<UsbPwrSpec> {
        FastRegAccessW::new(self, 31)
    }
}
#[doc = "USB3 Global power.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_pwr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_pwr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbPwrSpec;
impl crate::RegisterSpec for UsbPwrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_pwr::R`](R) reader structure"]
impl crate::Readable for UsbPwrSpec {}
#[doc = "`write(|w| ..)` method takes [`usb_pwr::W`](W) writer structure"]
impl crate::Writable for UsbPwrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets usb_pwr to value 0"]
impl crate::Resettable for UsbPwrSpec {
    const RESET_VALUE: u32 = 0;
}
