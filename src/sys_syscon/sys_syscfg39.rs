#[doc = "Register `sys_syscfg39` reader"]
pub type R = crate::R<SysSyscfg39Spec>;
#[doc = "Register `sys_syscfg39` writer"]
pub type W = crate::W<SysSyscfg39Spec>;
#[doc = "Field `u1_i2c_ic_en` reader - I2C interface enable."]
pub type U1I2cIcEnR = crate::BitReader;
#[doc = "Field `u1_sdio_data_strobe_phase_ctrl` reader - Data strobe delay chain select."]
pub type U1SdioDataStrobePhaseCtrlR = crate::FieldReader;
#[doc = "Field `u1_sdio_data_strobe_phase_ctrl` writer - Data strobe delay chain select."]
pub type U1SdioDataStrobePhaseCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u1_sdio_hbig_endian` reader - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
pub type U1SdioHbigEndianR = crate::BitReader;
#[doc = "Field `u1_sdio_hbig_endian` writer - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
pub type U1SdioHbigEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_sdio_m_hbig_endian` reader - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
pub type U1SdioMHbigEndianR = crate::BitReader;
#[doc = "Field `u1_sdio_m_hbig_endian` writer - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
pub type U1SdioMHbigEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_reset_ctrl_clr_reset_status` reader - "]
pub type U1ResetCtrlClrResetStatusR = crate::BitReader;
#[doc = "Field `u1_reset_ctrl_clr_reset_status` writer - "]
pub type U1ResetCtrlClrResetStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_reset_ctrl_pll_timecnt_finish` reader - "]
pub type U1ResetCtrlPllTimecntFinishR = crate::BitReader;
#[doc = "Field `u1_reset_ctrl_rstn_sw` reader - "]
pub type U1ResetCtrlRstnSwR = crate::BitReader;
#[doc = "Field `u1_reset_ctrl_rstn_sw` writer - "]
pub type U1ResetCtrlRstnSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_reset_ctrl_sys_reset_status` reader - "]
pub type U1ResetCtrlSysResetStatusR = crate::FieldReader;
#[doc = "Field `u2_i2c_ic_en` reader - I2C interface enable."]
pub type U2I2cIcEnR = crate::BitReader;
#[doc = "Field `u3_i2c_ic_en` reader - I2C interface enable."]
pub type U3I2cIcEnR = crate::BitReader;
#[doc = "Field `u4_i2c_ic_en` reader - I2C interface enable."]
pub type U4I2cIcEnR = crate::BitReader;
#[doc = "Field `u5_i2c_ic_en` reader - I2C interface enable."]
pub type U5I2cIcEnR = crate::BitReader;
#[doc = "Field `u6_i2c_ic_en` reader - I2C interface enable."]
pub type U6I2cIcEnR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - I2C interface enable."]
    #[inline(always)]
    pub fn u1_i2c_ic_en(&self) -> U1I2cIcEnR {
        U1I2cIcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Data strobe delay chain select."]
    #[inline(always)]
    pub fn u1_sdio_data_strobe_phase_ctrl(&self) -> U1SdioDataStrobePhaseCtrlR {
        U1SdioDataStrobePhaseCtrlR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
    #[inline(always)]
    pub fn u1_sdio_hbig_endian(&self) -> U1SdioHbigEndianR {
        U1SdioHbigEndianR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
    #[inline(always)]
    pub fn u1_sdio_m_hbig_endian(&self) -> U1SdioMHbigEndianR {
        U1SdioMHbigEndianR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn u1_reset_ctrl_clr_reset_status(&self) -> U1ResetCtrlClrResetStatusR {
        U1ResetCtrlClrResetStatusR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn u1_reset_ctrl_pll_timecnt_finish(&self) -> U1ResetCtrlPllTimecntFinishR {
        U1ResetCtrlPllTimecntFinishR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn u1_reset_ctrl_rstn_sw(&self) -> U1ResetCtrlRstnSwR {
        U1ResetCtrlRstnSwR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn u1_reset_ctrl_sys_reset_status(&self) -> U1ResetCtrlSysResetStatusR {
        U1ResetCtrlSysResetStatusR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - I2C interface enable."]
    #[inline(always)]
    pub fn u2_i2c_ic_en(&self) -> U2I2cIcEnR {
        U2I2cIcEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - I2C interface enable."]
    #[inline(always)]
    pub fn u3_i2c_ic_en(&self) -> U3I2cIcEnR {
        U3I2cIcEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I2C interface enable."]
    #[inline(always)]
    pub fn u4_i2c_ic_en(&self) -> U4I2cIcEnR {
        U4I2cIcEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I2C interface enable."]
    #[inline(always)]
    pub fn u5_i2c_ic_en(&self) -> U5I2cIcEnR {
        U5I2cIcEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I2C interface enable."]
    #[inline(always)]
    pub fn u6_i2c_ic_en(&self) -> U6I2cIcEnR {
        U6I2cIcEnR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:5 - Data strobe delay chain select."]
    #[inline(always)]
    #[must_use]
    pub fn u1_sdio_data_strobe_phase_ctrl(
        &mut self,
    ) -> U1SdioDataStrobePhaseCtrlW<SysSyscfg39Spec> {
        U1SdioDataStrobePhaseCtrlW::new(self, 1)
    }
    #[doc = "Bit 6 - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
    #[inline(always)]
    #[must_use]
    pub fn u1_sdio_hbig_endian(&mut self) -> U1SdioHbigEndianW<SysSyscfg39Spec> {
        U1SdioHbigEndianW::new(self, 6)
    }
    #[doc = "Bit 7 - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
    #[inline(always)]
    #[must_use]
    pub fn u1_sdio_m_hbig_endian(&mut self) -> U1SdioMHbigEndianW<SysSyscfg39Spec> {
        U1SdioMHbigEndianW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn u1_reset_ctrl_clr_reset_status(
        &mut self,
    ) -> U1ResetCtrlClrResetStatusW<SysSyscfg39Spec> {
        U1ResetCtrlClrResetStatusW::new(self, 8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn u1_reset_ctrl_rstn_sw(&mut self) -> U1ResetCtrlRstnSwW<SysSyscfg39Spec> {
        U1ResetCtrlRstnSwW::new(self, 10)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 156\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg39::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg39::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg39Spec;
impl crate::RegisterSpec for SysSyscfg39Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg39::R`](R) reader structure"]
impl crate::Readable for SysSyscfg39Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg39::W`](W) writer structure"]
impl crate::Writable for SysSyscfg39Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg39 to value 0x0400"]
impl crate::Resettable for SysSyscfg39Spec {
    const RESET_VALUE: u32 = 0x0400;
}
