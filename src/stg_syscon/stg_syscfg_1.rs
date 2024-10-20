#[doc = "Register `stg_syscfg_1` reader"]
pub type R = crate::R<StgSyscfg1Spec>;
#[doc = "Register `stg_syscfg_1` writer"]
pub type W = crate::W<StgSyscfg1Spec>;
#[doc = "Field `u0_usb_lowest_belt` reader - LTM interface to software"]
pub type U0UsbLowestBeltR = crate::FieldReader<u16>;
#[doc = "Field `u0_usb_ltm_host_req` reader - LTM interface to software"]
pub type U0UsbLtmHostReqR = crate::BitReader;
#[doc = "Field `u0_usb_ltm_host_req_halt` reader - LTM interface to software"]
pub type U0UsbLtmHostReqHaltR = crate::BitReader;
#[doc = "Field `u0_usb_ltm_host_req_halt` writer - LTM interface to software"]
pub type U0UsbLtmHostReqHaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_mdctrl_clk_sel` reader - "]
pub type U0UsbMdctrlClkSelR = crate::BitReader;
#[doc = "Field `u0_usb_mdctrl_clk_sel` writer - "]
pub type U0UsbMdctrlClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_mdctrl_clk_status` reader - "]
pub type U0UsbMdctrlClkStatusR = crate::BitReader;
#[doc = "Field `u0_usb_mode_strap` reader - Can onlly be changed when pwrup_rst_n is low"]
pub type U0UsbModeStrapR = crate::FieldReader;
#[doc = "Field `u0_usb_mode_strap` writer - Can onlly be changed when pwrup_rst_n is low"]
pub type U0UsbModeStrapW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `u0_usb_otg_suspendm` reader - "]
pub type U0UsbOtgSuspendmR = crate::BitReader;
#[doc = "Field `u0_usb_otg_suspendm` writer - "]
pub type U0UsbOtgSuspendmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_otg_suspendm_byps` reader - "]
pub type U0UsbOtgSuspendmBypsR = crate::BitReader;
#[doc = "Field `u0_usb_otg_suspendm_byps` writer - "]
pub type U0UsbOtgSuspendmBypsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_phy_bvalid` reader - "]
pub type U0UsbPhyBvalidR = crate::BitReader;
#[doc = "Field `u0_usb_pll_en` reader - "]
pub type U0UsbPllEnR = crate::BitReader;
#[doc = "Field `u0_usb_pll_en` writer - "]
pub type U0UsbPllEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_refclk_mode` reader - "]
pub type U0UsbRefclkModeR = crate::BitReader;
#[doc = "Field `u0_usb_refclk_mode` writer - "]
pub type U0UsbRefclkModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_rid_a_comp_sts` reader - "]
pub type U0CdnUsbRidACompStsR = crate::BitReader;
#[doc = "Field `u0_cdn_usb_rid_a_comp_sts` writer - "]
pub type U0CdnUsbRidACompStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_rid_b_comp_sts` reader - "]
pub type U0CdnUsbRidBCompStsR = crate::BitReader;
#[doc = "Field `u0_cdn_usb_rid_b_comp_sts` writer - "]
pub type U0CdnUsbRidBCompStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_rid_c_comp_sts` reader - "]
pub type U0CdnUsbRidCCompStsR = crate::BitReader;
#[doc = "Field `u0_cdn_usb_rid_c_comp_sts` writer - "]
pub type U0CdnUsbRidCCompStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_rid_float_comp_en` reader - "]
pub type U0UsbRidFloatCompEnR = crate::BitReader;
#[doc = "Field `u0_usb_rid_float_comp_sts` reader - "]
pub type U0UsbRidFloatCompStsR = crate::BitReader;
#[doc = "Field `u0_usb_rid_float_comp_sts` writer - "]
pub type U0UsbRidFloatCompStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_rid_gnd_comp_sts` reader - "]
pub type U0UsbRidGndCompStsR = crate::BitReader;
#[doc = "Field `u0_usb_rid_gnd_comp_sts` writer - "]
pub type U0UsbRidGndCompStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_rid_nonfloat_comp_en` reader - "]
pub type U0UsbRidNonfloatCompEnR = crate::BitReader;
#[doc = "Field `u0_usb_rx_dm` reader - "]
pub type U0UsbRxDmR = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - LTM interface to software"]
    #[inline(always)]
    pub fn u0_usb_lowest_belt(&self) -> U0UsbLowestBeltR {
        U0UsbLowestBeltR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - LTM interface to software"]
    #[inline(always)]
    pub fn u0_usb_ltm_host_req(&self) -> U0UsbLtmHostReqR {
        U0UsbLtmHostReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LTM interface to software"]
    #[inline(always)]
    pub fn u0_usb_ltm_host_req_halt(&self) -> U0UsbLtmHostReqHaltR {
        U0UsbLtmHostReqHaltR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn u0_usb_mdctrl_clk_sel(&self) -> U0UsbMdctrlClkSelR {
        U0UsbMdctrlClkSelR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn u0_usb_mdctrl_clk_status(&self) -> U0UsbMdctrlClkStatusR {
        U0UsbMdctrlClkStatusR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Can onlly be changed when pwrup_rst_n is low"]
    #[inline(always)]
    pub fn u0_usb_mode_strap(&self) -> U0UsbModeStrapR {
        U0UsbModeStrapR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn u0_usb_otg_suspendm(&self) -> U0UsbOtgSuspendmR {
        U0UsbOtgSuspendmR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn u0_usb_otg_suspendm_byps(&self) -> U0UsbOtgSuspendmBypsR {
        U0UsbOtgSuspendmBypsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn u0_usb_phy_bvalid(&self) -> U0UsbPhyBvalidR {
        U0UsbPhyBvalidR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn u0_usb_pll_en(&self) -> U0UsbPllEnR {
        U0UsbPllEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn u0_usb_refclk_mode(&self) -> U0UsbRefclkModeR {
        U0UsbRefclkModeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn u0_cdn_usb_rid_a_comp_sts(&self) -> U0CdnUsbRidACompStsR {
        U0CdnUsbRidACompStsR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn u0_cdn_usb_rid_b_comp_sts(&self) -> U0CdnUsbRidBCompStsR {
        U0CdnUsbRidBCompStsR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn u0_cdn_usb_rid_c_comp_sts(&self) -> U0CdnUsbRidCCompStsR {
        U0CdnUsbRidCCompStsR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn u0_usb_rid_float_comp_en(&self) -> U0UsbRidFloatCompEnR {
        U0UsbRidFloatCompEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn u0_usb_rid_float_comp_sts(&self) -> U0UsbRidFloatCompStsR {
        U0UsbRidFloatCompStsR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn u0_usb_rid_gnd_comp_sts(&self) -> U0UsbRidGndCompStsR {
        U0UsbRidGndCompStsR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn u0_usb_rid_nonfloat_comp_en(&self) -> U0UsbRidNonfloatCompEnR {
        U0UsbRidNonfloatCompEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn u0_usb_rx_dm(&self) -> U0UsbRxDmR {
        U0UsbRxDmR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - LTM interface to software"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_ltm_host_req_halt(&mut self) -> U0UsbLtmHostReqHaltW<StgSyscfg1Spec> {
        U0UsbLtmHostReqHaltW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_mdctrl_clk_sel(&mut self) -> U0UsbMdctrlClkSelW<StgSyscfg1Spec> {
        U0UsbMdctrlClkSelW::new(self, 14)
    }
    #[doc = "Bits 16:18 - Can onlly be changed when pwrup_rst_n is low"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_mode_strap(&mut self) -> U0UsbModeStrapW<StgSyscfg1Spec> {
        U0UsbModeStrapW::new(self, 16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_otg_suspendm(&mut self) -> U0UsbOtgSuspendmW<StgSyscfg1Spec> {
        U0UsbOtgSuspendmW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_otg_suspendm_byps(&mut self) -> U0UsbOtgSuspendmBypsW<StgSyscfg1Spec> {
        U0UsbOtgSuspendmBypsW::new(self, 20)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_pll_en(&mut self) -> U0UsbPllEnW<StgSyscfg1Spec> {
        U0UsbPllEnW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_refclk_mode(&mut self) -> U0UsbRefclkModeW<StgSyscfg1Spec> {
        U0UsbRefclkModeW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_rid_a_comp_sts(&mut self) -> U0CdnUsbRidACompStsW<StgSyscfg1Spec> {
        U0CdnUsbRidACompStsW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_rid_b_comp_sts(&mut self) -> U0CdnUsbRidBCompStsW<StgSyscfg1Spec> {
        U0CdnUsbRidBCompStsW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_rid_c_comp_sts(&mut self) -> U0CdnUsbRidCCompStsW<StgSyscfg1Spec> {
        U0CdnUsbRidCCompStsW::new(self, 26)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_rid_float_comp_sts(&mut self) -> U0UsbRidFloatCompStsW<StgSyscfg1Spec> {
        U0UsbRidFloatCompStsW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_rid_gnd_comp_sts(&mut self) -> U0UsbRidGndCompStsW<StgSyscfg1Spec> {
        U0UsbRidGndCompStsW::new(self, 29)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg1Spec;
impl crate::RegisterSpec for StgSyscfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_1::R`](R) reader structure"]
impl crate::Readable for StgSyscfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_1::W`](W) writer structure"]
impl crate::Writable for StgSyscfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_1 to value 0x2000"]
impl crate::Resettable for StgSyscfg1Spec {
    const RESET_VALUE: u32 = 0x2000;
}
