#[doc = "Register `aon_syscfg_10` reader"]
pub type R = crate::R<AonSyscfg10Spec>;
#[doc = "Register `aon_syscfg_10` writer"]
pub type W = crate::W<AonSyscfg10Spec>;
#[doc = "Field `u0_otpc_fl_sec_boot_lmt` reader - "]
pub type U0OtpcFlSecBootLmtR = crate::BitReader;
#[doc = "Field `u0_otpc_fl_xip` reader - "]
pub type U0OtpcFlXipR = crate::BitReader;
#[doc = "Field `u0_otpc_load_busy` reader - "]
pub type U0OtpcLoadBusyR = crate::BitReader;
#[doc = "Field `u0_reset_ctrl_clr_reset_status` reader - "]
pub type U0ResetCtrlClrResetStatusR = crate::BitReader;
#[doc = "Field `u0_reset_ctrl_clr_reset_status` writer - "]
pub type U0ResetCtrlClrResetStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_reset_ctrl_pll_timecnt_finish` reader - "]
pub type U0ResetCtrlPllTimecntFinishR = crate::BitReader;
#[doc = "Field `u0_reset_ctrl_rstn_sw` reader - "]
pub type U0ResetCtrlRstnSwR = crate::BitReader;
#[doc = "Field `u0_reset_ctrl_rstn_sw` writer - "]
pub type U0ResetCtrlRstnSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_reset_ctrl_sys_reset_status` reader - "]
pub type U0ResetCtrlSysResetStatusR = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn u0_otpc_fl_sec_boot_lmt(&self) -> U0OtpcFlSecBootLmtR {
        U0OtpcFlSecBootLmtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn u0_otpc_fl_xip(&self) -> U0OtpcFlXipR {
        U0OtpcFlXipR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn u0_otpc_load_busy(&self) -> U0OtpcLoadBusyR {
        U0OtpcLoadBusyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn u0_reset_ctrl_clr_reset_status(&self) -> U0ResetCtrlClrResetStatusR {
        U0ResetCtrlClrResetStatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn u0_reset_ctrl_pll_timecnt_finish(&self) -> U0ResetCtrlPllTimecntFinishR {
        U0ResetCtrlPllTimecntFinishR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn u0_reset_ctrl_rstn_sw(&self) -> U0ResetCtrlRstnSwR {
        U0ResetCtrlRstnSwR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn u0_reset_ctrl_sys_reset_status(&self) -> U0ResetCtrlSysResetStatusR {
        U0ResetCtrlSysResetStatusR::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn u0_reset_ctrl_clr_reset_status(
        &mut self,
    ) -> U0ResetCtrlClrResetStatusW<AonSyscfg10Spec> {
        U0ResetCtrlClrResetStatusW::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn u0_reset_ctrl_rstn_sw(&mut self) -> U0ResetCtrlRstnSwW<AonSyscfg10Spec> {
        U0ResetCtrlRstnSwW::new(self, 5)
    }
}
#[doc = "AON SYSCONSAIF SYSCFG 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonSyscfg10Spec;
impl crate::RegisterSpec for AonSyscfg10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_syscfg_10::R`](R) reader structure"]
impl crate::Readable for AonSyscfg10Spec {}
#[doc = "`write(|w| ..)` method takes [`aon_syscfg_10::W`](W) writer structure"]
impl crate::Writable for AonSyscfg10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets aon_syscfg_10 to value 0x20"]
impl crate::Resettable for AonSyscfg10Spec {
    const RESET_VALUE: u32 = 0x20;
}
