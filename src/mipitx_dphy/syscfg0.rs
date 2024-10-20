#[doc = "Register `syscfg0` reader"]
pub type R = crate::R<Syscfg0Spec>;
#[doc = "Register `syscfg0` writer"]
pub type W = crate::W<Syscfg0Spec>;
#[doc = "Field `aon_power_ready_n` reader - Always-on Power Ready: u0_mipitx_dphy_AON_POWER_READY_N"]
pub type AonPowerReadyNR = crate::BitReader;
#[doc = "Field `aon_power_ready_n` writer - Always-on Power Ready: u0_mipitx_dphy_AON_POWER_READY_N"]
pub type AonPowerReadyNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cklan` reader - Configure CKLAN: u0_mipitx_dphy_CFG_CKLANE_SET"]
pub type CklanR = crate::FieldReader;
#[doc = "Field `cklan` writer - Configure CKLAN: u0_mipitx_dphy_CFG_CKLANE_SET"]
pub type CklanW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `databus16_sel` reader - Configure DATABUS16_SEL: u0_mipitx_dphy_CFG_DATABUS16_SEL"]
pub type Databus16SelR = crate::BitReader;
#[doc = "Field `databus16_sel` writer - Configure DATABUS16_SEL: u0_mipitx_dphy_CFG_DATABUS16_SEL"]
pub type Databus16SelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dpdn_swap` reader - Configure DPDN_SWAP: u0_mipitx_dphy_CFG_DPDN_SWAP"]
pub type DpdnSwapR = crate::FieldReader;
#[doc = "Field `dpdn_swap` writer - Configure DPDN_SWAP: u0_mipitx_dphy_CFG_DPDN_SWAP"]
pub type DpdnSwapW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `swap_sel(_l0,_l1,_l2,_l3,_l4)` reader - Configure SWAP_SEL: u0_mipitx_dphy_CFG_L\\[n\\]_SWAP_SEL"]
pub type SwapSelR = crate::FieldReader;
#[doc = "Field `swap_sel(_l0,_l1,_l2,_l3,_l4)` writer - Configure SWAP_SEL: u0_mipitx_dphy_CFG_L\\[n\\]_SWAP_SEL"]
pub type SwapSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Always-on Power Ready: u0_mipitx_dphy_AON_POWER_READY_N"]
    #[inline(always)]
    pub fn aon_power_ready_n(&self) -> AonPowerReadyNR {
        AonPowerReadyNR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Configure CKLAN: u0_mipitx_dphy_CFG_CKLANE_SET"]
    #[inline(always)]
    pub fn cklan(&self) -> CklanR {
        CklanR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Configure DATABUS16_SEL: u0_mipitx_dphy_CFG_DATABUS16_SEL"]
    #[inline(always)]
    pub fn databus16_sel(&self) -> Databus16SelR {
        Databus16SelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:11 - Configure DPDN_SWAP: u0_mipitx_dphy_CFG_DPDN_SWAP"]
    #[inline(always)]
    pub fn dpdn_swap(&self) -> DpdnSwapR {
        DpdnSwapR::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Configure SWAP_SEL: u0_mipitx_dphy_CFG_L\\[n\\]_SWAP_SEL"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `swap_sel_l0` field"]
    #[inline(always)]
    pub fn swap_sel(&self, n: u8) -> SwapSelR {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        SwapSelR::new(((self.bits >> (n * 3 + 12)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Configure SWAP_SEL: u0_mipitx_dphy_CFG_L\\[n\\]_SWAP_SEL"]
    #[inline(always)]
    pub fn swap_sel_iter(&self) -> impl Iterator<Item = SwapSelR> + '_ {
        (0..5).map(move |n| SwapSelR::new(((self.bits >> (n * 3 + 12)) & 7) as u8))
    }
    #[doc = "Bits 12:14 - Configure SWAP_SEL: u0_mipitx_dphy_CFG_L\\[n\\]_SWAP_SEL"]
    #[inline(always)]
    pub fn swap_sel_l0(&self) -> SwapSelR {
        SwapSelR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Configure SWAP_SEL: u0_mipitx_dphy_CFG_L\\[n\\]_SWAP_SEL"]
    #[inline(always)]
    pub fn swap_sel_l1(&self) -> SwapSelR {
        SwapSelR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Configure SWAP_SEL: u0_mipitx_dphy_CFG_L\\[n\\]_SWAP_SEL"]
    #[inline(always)]
    pub fn swap_sel_l2(&self) -> SwapSelR {
        SwapSelR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Configure SWAP_SEL: u0_mipitx_dphy_CFG_L\\[n\\]_SWAP_SEL"]
    #[inline(always)]
    pub fn swap_sel_l3(&self) -> SwapSelR {
        SwapSelR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Configure SWAP_SEL: u0_mipitx_dphy_CFG_L\\[n\\]_SWAP_SEL"]
    #[inline(always)]
    pub fn swap_sel_l4(&self) -> SwapSelR {
        SwapSelR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Always-on Power Ready: u0_mipitx_dphy_AON_POWER_READY_N"]
    #[inline(always)]
    #[must_use]
    pub fn aon_power_ready_n(&mut self) -> AonPowerReadyNW<Syscfg0Spec> {
        AonPowerReadyNW::new(self, 0)
    }
    #[doc = "Bits 1:5 - Configure CKLAN: u0_mipitx_dphy_CFG_CKLANE_SET"]
    #[inline(always)]
    #[must_use]
    pub fn cklan(&mut self) -> CklanW<Syscfg0Spec> {
        CklanW::new(self, 1)
    }
    #[doc = "Bit 6 - Configure DATABUS16_SEL: u0_mipitx_dphy_CFG_DATABUS16_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn databus16_sel(&mut self) -> Databus16SelW<Syscfg0Spec> {
        Databus16SelW::new(self, 6)
    }
    #[doc = "Bits 7:11 - Configure DPDN_SWAP: u0_mipitx_dphy_CFG_DPDN_SWAP"]
    #[inline(always)]
    #[must_use]
    pub fn dpdn_swap(&mut self) -> DpdnSwapW<Syscfg0Spec> {
        DpdnSwapW::new(self, 7)
    }
    #[doc = "Configure SWAP_SEL: u0_mipitx_dphy_CFG_L\\[n\\]_SWAP_SEL"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `swap_sel_l0` field"]
    #[inline(always)]
    #[must_use]
    pub fn swap_sel(&mut self, n: u8) -> SwapSelW<Syscfg0Spec> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        SwapSelW::new(self, n * 3 + 12)
    }
    #[doc = "Bits 12:14 - Configure SWAP_SEL: u0_mipitx_dphy_CFG_L\\[n\\]_SWAP_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn swap_sel_l0(&mut self) -> SwapSelW<Syscfg0Spec> {
        SwapSelW::new(self, 12)
    }
    #[doc = "Bits 15:17 - Configure SWAP_SEL: u0_mipitx_dphy_CFG_L\\[n\\]_SWAP_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn swap_sel_l1(&mut self) -> SwapSelW<Syscfg0Spec> {
        SwapSelW::new(self, 15)
    }
    #[doc = "Bits 18:20 - Configure SWAP_SEL: u0_mipitx_dphy_CFG_L\\[n\\]_SWAP_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn swap_sel_l2(&mut self) -> SwapSelW<Syscfg0Spec> {
        SwapSelW::new(self, 18)
    }
    #[doc = "Bits 21:23 - Configure SWAP_SEL: u0_mipitx_dphy_CFG_L\\[n\\]_SWAP_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn swap_sel_l3(&mut self) -> SwapSelW<Syscfg0Spec> {
        SwapSelW::new(self, 21)
    }
    #[doc = "Bits 24:26 - Configure SWAP_SEL: u0_mipitx_dphy_CFG_L\\[n\\]_SWAP_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn swap_sel_l4(&mut self) -> SwapSelW<Syscfg0Spec> {
        SwapSelW::new(self, 24)
    }
}
#[doc = "MIPITX DPHY SYSCFG 0: mipitx_apbifsaif_syscfg_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg0Spec;
impl crate::RegisterSpec for Syscfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg0::R`](R) reader structure"]
impl crate::Readable for Syscfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg0::W`](W) writer structure"]
impl crate::Writable for Syscfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets syscfg0 to value 0x0111_a021"]
impl crate::Resettable for Syscfg0Spec {
    const RESET_VALUE: u32 = 0x0111_a021;
}
